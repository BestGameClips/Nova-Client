package nova.nativeclient;
import java.lang.instrument.Instrumentation;
import java.lang.management.ManagementFactory;
public final class Bootstrap {
 private Bootstrap() {}
 public static void premain(String options, Instrumentation instrumentation) {
  System.out.println("[Nova] Native bootstrap loaded; Minecraft class transformations not installed.");
  System.out.println("[Nova] JVM max heap: "+Runtime.getRuntime().maxMemory()/1048576+" MiB");
 }
}
