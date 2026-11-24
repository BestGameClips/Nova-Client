package nova.client;
import net.minecraft.client.Minecraft;
import com.google.gson.*;
import java.io.*;
import java.nio.*;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.util.Properties;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicBoolean;
public final class Presence {
 private static final Path CONFIG=Path.of("config","nova-discord.properties");
 private static final ExecutorService IO=Executors.newSingleThreadExecutor(r->{Thread t=new Thread(r,"Nova Discord");t.setDaemon(true);return t;});
 private static final AtomicBoolean BUSY=new AtomicBoolean();
 private static long next=0,started=System.currentTimeMillis()/1000;
 private static RandomAccessFile pipe;
 private static String connectedId="";
 private static String lastMode="";
 public static void tick(Minecraft mc){
  long now=System.currentTimeMillis();if(now<next)return;next=now+15000;
  if(!BUSY.compareAndSet(false,true))return;
  try{
   String mode=mc.level==null?"Dans les menus":mc.hasSingleplayerServer()?"Monde solo":"Multijoueur";
   String server=mc.getCurrentServer()==null?"":mc.getCurrentServer().name;
   if(!mode.equals(lastMode)){started=now/1000;lastMode=mode;}
   long time=started;
   IO.execute(()->{try{update(mode,server,time);}catch(Exception e){close();}finally{BUSY.set(false);}});
  }catch(Exception e){BUSY.set(false);}
 }
 private static Properties config() throws IOException{
  Properties p=new Properties();
  if(!Files.exists(CONFIG)){Files.createDirectories(CONFIG.getParent());Files.writeString(CONFIG,"# Discord RPC opt-in\nenabled=false\napplicationId=\nshowServerName=false\nshowTimer=true\n",StandardOpenOption.CREATE_NEW);}
  try(var in=Files.newBufferedReader(CONFIG)){p.load(in);}return p;
 }
 private static void update(String mode,String server,long time)throws IOException{
  Properties p=config();String id=p.getProperty("applicationId","").trim();
  if(!Boolean.parseBoolean(p.getProperty("enabled","false"))||!id.matches("[0-9]{15,22}")){close();return;}
  if(!id.equals(connectedId))close();
  if(pipe==null){for(int i=0;i<10;i++){try{pipe=new RandomAccessFile("\\\\.\\pipe\\discord-ipc-"+i,"rw");break;}catch(IOException ignored){}}
   if(pipe==null)return;
   JsonObject hello=new JsonObject();hello.addProperty("v",1);hello.addProperty("client_id",id);send(0,hello);connectedId=id;
  }
  JsonObject activity=new JsonObject();activity.addProperty("details",mode);
  if(mode.equals("Multijoueur")){String label=Boolean.parseBoolean(p.getProperty("showServerName","false"))&&!server.isBlank()?server:"Serveur privé";activity.addProperty("state",label.substring(0,Math.min(128,label.length())));}
  else activity.addProperty("state","Minecraft 26.3");
  if(Boolean.parseBoolean(p.getProperty("showTimer","true"))){JsonObject ts=new JsonObject();ts.addProperty("start",time);activity.add("timestamps",ts);}
  JsonObject args=new JsonObject();args.addProperty("pid",ProcessHandle.current().pid());args.add("activity",activity);
  JsonObject cmd=new JsonObject();cmd.addProperty("cmd","SET_ACTIVITY");cmd.add("args",args);cmd.addProperty("nonce",java.util.UUID.randomUUID().toString());send(1,cmd);
  // Reconnect on the next update so responses cannot accumulate in the IPC buffer.
  // Keep the connection alive between updates; drain available frames without blocking.
 }
 private static void send(int op,JsonObject data)throws IOException{
  byte[] bytes=data.toString().getBytes(StandardCharsets.UTF_8);
  byte[] header=ByteBuffer.allocate(8).order(ByteOrder.LITTLE_ENDIAN).putInt(op).putInt(bytes.length).array();
  pipe.write(header);pipe.write(bytes);
 }
 private static void close(){if(pipe!=null){try{pipe.close();}catch(IOException ignored){}pipe=null;}connectedId="";}
}
