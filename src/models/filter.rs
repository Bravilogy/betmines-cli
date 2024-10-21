use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Rule {
    pub target: Option<String>,
    pub probability: Option<String>,
    pub time: Option<String>,

    #[serde(rename = "valueToCompare")]
    pub value_to_compare: Option<f32>,

    #[serde(rename = "trendStats")]
    pub trend_stats: Option<String>,

    #[serde(rename = "comparatorType")]
    pub comparator_type: Option<String>,

    #[serde(rename = "type")]
    pub rule_type: Option<String>,
}

impl Eq for Rule {}
impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.comparator_type.hash(state);
        self.target.hash(state);
        self.rule_type.hash(state);
        self.value_to_compare.map(|v| v.to_bits()).hash(state);
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.comparator_type == other.comparator_type
            && self.target == other.target
            && self.rule_type == other.rule_type
            && self.trend_stats == other.trend_stats
            && self.probability == other.probability
            && self.time == other.time
            && self.value_to_compare.map(|v| v.to_bits())
                == other.value_to_compare.map(|v| v.to_bits())
    }
}

impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.comparator_type
            .cmp(&other.comparator_type)
            .then(self.target.cmp(&other.target))
            .then(self.rule_type.cmp(&other.rule_type))
            .then(self.probability.cmp(&other.probability))
            .then(self.time.cmp(&other.time))
            .then(self.trend_stats.cmp(&other.trend_stats))
            .then(
                self.value_to_compare
                    .partial_cmp(&other.value_to_compare)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
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

        let mut sorted_rules = self.rules.clone();
        sorted_rules.sort();

        sorted_rules.hash(state);
    }
}

impl PartialEq for Filter {
    fn eq(&self, other: &Self) -> bool {
        let mut self_rules = self.rules.clone();
        let mut other_rules = other.rules.clone();

        self_rules.sort();
        other_rules.sort();

        self.desired_outcome == other.desired_outcome && self_rules == other_rules
    }
}
