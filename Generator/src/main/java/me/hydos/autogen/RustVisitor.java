package me.hydos.autogen;

import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.FieldVisitor;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

public class RustVisitor extends ClassVisitor {

    public String className;
    public String rawName;
    public FileWriter rust;

    public RustVisitor(File outputFile, String rawName) {
        super(Opcodes.ASM9);
        try {
            if (!outputFile.getParentFile().exists())
                outputFile.getParentFile().mkdirs();
            if (!outputFile.exists())
                outputFile.createNewFile();

            this.rust = new FileWriter(outputFile);
            this.rawName = rawName;
            this.className = processName(rawName);

            rust.write("use jni::JNIEnv;\n");
            rust.write("\n");
            rust.write("struct " + className + "<'a> (JNIEnv<'a>);\n");
            rust.write("\n");
            rust.write("impl " + className + "<'_> {\n");
            rust.write("\n");

        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private String processName(String className) {
        String[] split = className.split("/");
        return split[split.length - 1];
    }

    @Override
    public MethodVisitor visitMethod(int access, String name, String descriptor, String signature, String[] exceptions) {
        return super.visitMethod(access, name, descriptor, signature, exceptions);
    }

    @Override
    public FieldVisitor visitField(int access, String name, String descriptor, String signature, Object value) {
        if (access > Opcodes.ACC_STATIC) {
            System.out.println(name + " is static!");
        }
        return super.visitField(access, name, descriptor, signature, value);
    }

    @Override
    public void visitEnd() {
        try {
            rust.write("}\n");
            rust.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
