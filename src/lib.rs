pub type FeatureVersion = u16;
pub type OptionalFeatureVersion = Option<u16>; //This is a feature that didn't always exist

#[derive(Clone, Debug, Default)]
pub struct FeatureVersionBounds {
    pub min_version: FeatureVersion,
    pub max_version: FeatureVersion,
    pub default_current_version: FeatureVersion,
}

impl FeatureVersionBounds {
    /// Will get a protocol error if the version is unknown
    pub fn check_version(&self, version: FeatureVersion) -> bool {
        version >= self.min_version && version <= self.max_version
    }
}