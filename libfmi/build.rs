fn main() {
    cc::Build::new().file("src/logger.c").compile("logger");
}
