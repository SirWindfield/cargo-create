use petgraph::{
    algo::{toposort, Cycle},
    graphmap::DiGraphMap,
};
use serde::Deserialize;
use std::borrow::Cow;

/// The configuration file of a template repository.
#[derive(Debug, Deserialize)]
pub struct TemplateConfig<'a> {
    /// The features of the template.
    #[serde(borrow)]
    pub features: Vec<Feature<'a>>,
}

impl<'a> TemplateConfig<'a> {
    /// Performs a topological sort of the features.
    ///
    /// This function is used to detect cyclic dependencies inside the
    /// configuration file and early-abort in those cases.
    pub fn do_topological_sort_of_features(&self) -> Result<Vec<&str>, Cycle<&str>> {
        let mut g = DiGraphMap::new();
        for feature in &self.features {
            g.add_node(feature.name);
            if let Some(include) = &feature.include {
                for workflow in include {
                    g.add_edge(feature.name, workflow, 1);
                }
            }
        }

        toposort(&g, None)
    }

    pub fn feature(&self, name: &str) -> Option<&Feature<'a>> {
        self.features.iter().find(|&f| f.name == name)
    }
}

/// A template feature.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Feature<'a> {
    /// The name of the feature. Has to be unique inside a repository.
    pub name: &'a str,
    /// The files that get included and processed if the feature is being used.
    #[serde(borrow)]
    pub files: Option<Vec<FileEntry<'a>>>,
    /// Features that should get included if this one is being used.
    pub include: Option<Vec<&'a str>>,
}

impl<'a> Feature<'a> {
    /// Returns all files to include with this feature.
    ///
    /// This does merge all files plus the files that get included from all
    /// other workflows.
    pub fn files_to_include(&self) -> Vec<FileEntry<'a>> {
        let mut vec = Vec::new();

        if let Some(files) = &self.files {
            vec.extend(files);
        }

        vec
    }

    /// Returns a list of features that are part of a feature.
    ///
    /// Features including other features via the `include` keywords are called
    /// super-features.
    ///
    /// # Arguments
    ///
    /// - `all_features`: A slice of all features that have been defined.
    pub fn get_features_for_feature<'b>(
        &'b self,
        all_features: &'b [&'b Feature<'b>],
    ) -> Cow<'b, [&'b Feature<'b>]> {
        if let Some(includes) = &self.include {
            // Collect all features that have been declared inside the `include` keyword if
            // this feature (`self`).
            let features_part_of_this_feature = all_features
                .iter()
                .filter(|feat| includes.contains(&feat.name))
                .copied()
                .collect::<Vec<_>>();

            // TODO: add cyclic dependency check?
            // For each feature that is part of this one, recursively call this
            // function to collect all features.
            let mut features = Vec::new();
            for &feat in features_part_of_this_feature.iter() {
                let f = feat.get_features_for_feature(all_features);
                match f {
                    Cow::Owned(feature_vec) => {
                        features.extend(feature_vec.into_iter());
                    }
                    Cow::Borrowed(feature_slice) => {
                        features.extend(feature_slice.iter().copied());
                    }
                }
            }

            features.push(self);

            // ERROR: returns reference of temporarely created value (obviously!)
            return Cow::Owned(features);
        }

        // Safety: all_features will always contain all features.
        let self_index = all_features
            .iter()
            .position(|feat| feat.name == self.name)
            .unwrap();
        let value = &all_features[self_index..=self_index];
        Cow::Borrowed(value)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum FileEntry<'a> {
    Complex(MoveFileEntry<'a>),
    Simple(&'a str),
}

impl Default for FileEntry<'_> {
    fn default() -> Self {
        Self::Simple(Default::default())
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct MoveFileEntry<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

#[cfg(test)]
mod tests {
    use super::TemplateConfig;

    #[test]
    fn test_de_rename_files() {
        let config = r#"
        [[features]]
        name = "one"
        files = [
            "simple.txt",
            { from = "complex.txt", to = "moved.txt" }
        ]
        "#;

        let config: TemplateConfig = toml::from_str(&config).unwrap();
        println!("{:#?}", config);
        assert!(true);
    }
}
