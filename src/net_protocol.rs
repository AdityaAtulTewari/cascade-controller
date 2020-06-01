use tokio::net::TcpListener;
use tokio::net::Tcp::ReadHalf;
use futures::channel::mpsc::UnboundedSender;
use tokio::io::{Result,Error,ErrorKind};
use futures::stream::poll_fn;


async fn msg_get(reader: ReadHalf, buf: &mut [u8;8]) -> Result<Vec<u8>>
{
  let n = reader.read(&mut buf).await?;
  if n != 8
  {
    return Err(Error::new(ErrorKind::InvalidData, "Failed to read 8 bytes from protocol."));
  }
  let size = u32::from_le_bytes(&buf[0..4]) + 8;
  let mut vec = Vec::with_capacity(size);
  vec.resize(size as usize, 0);
  for i in 0..8
  {
    vec[i] = buf[i];
  }
  let n = reader.read(&mut vec[8..size]).await?;
  if n != size - 8
  {
    return Err(Error::new(ErrorKind::InvalidData, "Failed to read the right number of bytes from protocol."));
  }
  return Ok(vec);
}

async fn msg_forward(reader: ReadHalf, sender: UnboundedSender<Result<Vec<u8>>>)
{
  let mut buf: [u8;8];
  loop
  {
    let msg = msg_get(reader, &mut buf);
    let suc = sender.unbounded_send(msg);
    match msg
    {
      Err(_) => {sender.close_channel(); return;}
      _      => {}
    }
    match suc
    {
      Err(x) => {println!("Failed to send: {:?}",x); return;}
      _      => {}
    }
  }
}
