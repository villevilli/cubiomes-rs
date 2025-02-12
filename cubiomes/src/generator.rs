enum McVersion {
    McUndef = cubiomes_sys::McVersion::McUndef,
}

pub struct Generator {
    generator: cubiomes_sys::Generator,
}

impl Generator {
    fn new(version: cubiomes_sys::MCVersion) -> Self {}
}
