plugins {
    java
}

group = "me.hydos"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.ow2.asm", "asm", "9.1")
}

tasks.getByName<Test>("test") {
    useJUnitPlatform()
}