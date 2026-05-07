pub fn get_map() -> rosu_map::Beatmap {
    rosu_map::from_path("./assets/test/mania_beatmap.osu").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_map();
        assert_eq!(result.artist, "Aiobahn +81");
    }
}