plugins {
    alias(libs.plugins.kotlin.jvm)
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.squareup.retrofit2:retrofit:2.12.0")
    implementation("com.squareup.retrofit2:converter-gson:2.12.0")
    implementation("io.github.cdimascio:dotenv-kotlin:6.3.1")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit5")
    testImplementation(libs.junit.jupiter.engine)
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(8)
    }
}

application {
    mainClass = "dev.resolvedor.AppKt"
}

tasks.named<Test>("test") {
    useJUnitPlatform()
}

tasks.withType<org.gradle.jvm.tasks.Jar> {
    manifest {
        attributes["Main-Class"] = "dev.resolvedor.AppKt"
    }

    from(
        configurations.runtimeClasspath.get().map { file ->
            if (file.isDirectory) file else zipTree(file)
        }
    )

    duplicatesStrategy = org.gradle.api.file.DuplicatesStrategy.EXCLUDE
}
