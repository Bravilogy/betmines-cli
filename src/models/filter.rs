use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Rule {
    #[serde(rename = "valueToCompare")]
    pub value_to_compare: Option<f32>,

    #[serde(rename = "comparatorType")]
    pub comparator_type: Option<String>,

    pub target: Option<String>,

    #[serde(rename = "type")]
    pub rule_type: Option<String>,
}

impl Eq for Rule {}
impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.comparator_type.hash(state);
        self.target.hash(state);
        self.rule_type.hash(state);

        if let Some(v) = self.value_to_compare {
            v.to_bits().hash(state);
        } else {
            None::<u64>.hash(state);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Filter {
    pub id: i32,
    pub roi: f32,

    #[serde(rename = "serviceName")]
    pub name: String,

    pub rules: Vec<Rule>,

    #[serde(rename = "totalPicks")]
    pub total_picks: u32,

    #[serde(rename = "successPercentage")]
    pub success_rate: f32,

    #[serde(rename = "desiredOutcome")]
    pub desired_outcome: Option<String>,

    #[serde(skip)]
    pub score: f64,
}

impl Eq for Filter {}
impl Hash for Filter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.desired_outcome.hash(state);
        self.rules.hash(state);
    }
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool {
        self.desired_outcome == other.desired_outcome && self.rules == other.rules
    }
}
