pub struct ProfileId(pub i64);
#[allow(dead_code)]
pub struct DotfileId(pub i64);

pub struct Profile {
    pub id: ProfileId,
}

#[allow(dead_code)]
pub struct Dotfile {
    pub id: DotfileId,
    pub source: String,
    pub description: Option<String>,
}

#[allow(dead_code)]
pub struct ProfileDotfiles {
    pub profile_id: ProfileId,
    pub dotfile_id: DotfileId,
}
