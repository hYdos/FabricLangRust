package me.hydos.autogen;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.function.Function;
import java.util.jar.JarFile;

public class BridgeGenerator {

    public static final Path MINECRAFT = Paths.get("/home/hydos/.gradle/caches/fabric-loom/21w19a-mapped-net.fabricmc.yarn-21w19a+build.6-v2/minecraft-21w19a-mapped-net.fabricmc.yarn-21w19a+build.6-v2.jar");
    public static final Path GENERATED_DIR = Paths.get("/home/hydos/IdeaProjects/hYdos/QuiltLangRust/RustBootstrap/src/generated");

    public static void main(String[] args) throws IOException {
        visitJar(MINECRAFT, BridgeGenerator::createVisitor);
    }

    private static ClassVisitor createVisitor(String name) {
        return new RustVisitor(new File(GENERATED_DIR.toString(), name + ".rs"), name);
    }

    public static void visitJar(Path jarPath, Function<String, ClassVisitor> visitor) throws IOException {
        JarFile jar = new JarFile(jarPath.toFile());
        jar.stream().forEach(entry -> {
            if (entry.getName().endsWith(".class")) {
                try (InputStream inputStream = jar.getInputStream(entry)) {
                    ClassReader reader = new ClassReader(inputStream);
                    reader.accept(visitor.apply(entry
                            .getName()
                            .replace(".class", "")
                    ), ClassReader.EXPAND_FRAMES);
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        });
    }
}
