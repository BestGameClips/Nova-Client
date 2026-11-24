package nova.client.mixin;
import net.minecraft.client.Minecraft;
import nova.client.Presence;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(Minecraft.class)
public abstract class PresenceMixin {
 @Inject(method="tick",at=@At("TAIL"))
 private void nova$presence(CallbackInfo ci){Presence.tick((Minecraft)(Object)this);}
}
