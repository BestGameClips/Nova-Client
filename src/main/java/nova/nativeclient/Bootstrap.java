package nova.nativeclient;
import java.lang.instrument.*;
import java.security.ProtectionDomain;
import java.lang.classfile.*;
import java.lang.classfile.instruction.*;
import java.lang.constant.*;
public final class Bootstrap {
 public static void premain(String options, Instrumentation inst) {
  inst.addTransformer(new ClassFileTransformer(){
   @Override public byte[] transform(Module m,ClassLoader l,String name,Class<?> c,ProtectionDomain p,byte[] bytes){
    if(!"net/minecraft/client/gui/screens/TitleScreen".equals(name)&&!"net/minecraft/client/gui/screens/LoadingOverlay".equals(name))return null;
    boolean title=name.endsWith("TitleScreen");
    try{
     ClassFile cf=ClassFile.of();
     return cf.transformClass(cf.parse(bytes),ClassTransform.transformingMethods(method->method.methodName().stringValue().equals(title?"extractBackground":"extractRenderState")&&method.methodType().stringValue().equals("(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V"),MethodTransform.transformingCode((builder,element)->{
      if(element instanceof ReturnInstruction){
       builder.aload(1).invokestatic(ClassDesc.of("nova.nativeclient.Theme"),title?"background":"loading",MethodTypeDesc.of(ConstantDescs.CD_void,ConstantDescs.CD_Object));
      }
      builder.with(element);
     })));
    }catch(Throwable e){System.err.println("[Nova] UI transformation failed: "+e.getClass().getSimpleName());return null;}
   }
  });
  System.out.println("[Nova] Native agent active");
 }
}
