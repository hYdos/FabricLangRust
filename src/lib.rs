//! these 2 impl's are placeholders and will be generated later on.
mod raw_jni;

struct MinecraftClient {}

struct MinecraftDedicatedServer {}


struct Minecraft {}
/// This is the way to interact with all things java related (most mods and Minecraft)
impl Minecraft {
    /// Calls LogManager.getLogger(logger_name).info(message);
    fn log(logger_name: &str, message: &str) {}

    /// Gets an instance of the minecraft client. this is should NOT be called on the server
    fn get_client() -> MinecraftClient {
        return MinecraftClient {};
    }

    /// Gets an instance of the minecraft dedicated server. this is should NOT be called on the client
    fn get_server() -> MinecraftDedicatedServer {
        return MinecraftDedicatedServer {};
    }
}

/// Called when the java ModInitializer is
trait ModInitializer {
    fn on_initialize();
}

/// Called when the java ServerModInitializer is
trait ServerModInitializer {
    fn on_initialize_server();
}

/// Called when the java ClientModInitializer is
trait ClientModInitializer {
    fn on_initialize_client();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}