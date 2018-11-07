pub struct FuzzySetResult {
    pub items: Vec<(String, f64)>
}

pub struct FuzzyBool {
    pub is_true: bool,
    pub value: f64,
}

pub struct FuzzySets {
    pub  start: InverseGrade,
    pub  triangles: Vec<Triangle>,
    pub  end: Grade,
}

pub struct Triangle {
    pub  name: String,
    pub x0: f64,
    pub x1: f64,
    pub x2: f64,
    pub clip: f64,
}

pub struct Grade {
    pub name: String,
    pub x0: f64,
    pub  x1: f64,
    pub  clip: f64,
}

pub struct InverseGrade {
    pub  name: String,
    pub x0: f64,
    pub x1: f64,
    pub clip: f64,
}

impl FuzzyBool {
    pub fn and(&self, b: FuzzyBool) -> FuzzyBool {
        FuzzyBool {
            is_true: self.is_true == true && b.is_true == true,
            value: self.value.min(b.value),
        }
    }

    pub fn or(&self, b: FuzzyBool) -> FuzzyBool {
        FuzzyBool {
            is_true: self.is_true == true && b.is_true == true,
            value: self.value.max(b.value),
        }
    }

    pub fn not(&self) -> FuzzyBool {
        FuzzyBool {
            is_true: self.is_true == true,
            value: 1.0 - self.value,
        }
    }

    pub fn then(&self, target: &FuzzySets, set: &str) -> FuzzySetResult {
        let mut res = FuzzySetResult {
            items: Vec::new(),
        };
        if target.contains(set) && self.is_true {
            res.items = vec!((String::from(set), self.value))
        }
        return res;
    }
}

impl FuzzySetResult {
    pub fn is(&self, set: &str) -> FuzzyBool {
        let mut result = FuzzyBool {
            is_true: false,
            value: 0.0,
        };
        let res: Vec<f64> = self.items.iter().filter(|item| item.0 == set).map(|f| f.1).collect();
        if res.len() == 1 {
            result.value = res[0];
            result.is_true = true;
        }
        return result;
    }
}

impl Triangle {
    pub fn fuzzify(&self, position: f64) -> f64 {
        let mut value: f64 = 0.0;
        if position >= self.x0 && position <= self.x1 {
            value = (position - self.x0) / (self.x1 - self.x0);
        } else if position >= self.x1 && position <= self.x2 {
            value = (self.x2 - position) / (self.x1 - self.x0);
        }
        if value > self.clip {
            value = self.clip;
        }
        return value;
    }
}

impl Grade {
    pub fn fuzzify(&self, position: f64) -> f64 {
        let mut value: f64 = 0.0;
        if position >= self.x1 {
            value = 1.0;
        } else if position <= self.x0 {
            value = 0.0;
        } else {
            value = (position - self.x0) / (self.x1 - self.x0);
        }
        if value > self.clip {
            value = self.clip;
        }
        return value;
    }
}

impl InverseGrade {
    pub fn fuzzify(&self, position: f64) -> f64 {
        let mut value: f64 = 0.0;
        if position <= self.x0 {
            value = 1.0;
        } else if position >= self.x1 {
            value = 0.0;
        } else {
            value = (self.x1 - position) / (self.x1 - self.x0);
        }
        if value > self.clip {
            value = self.clip;
        }
        return value;
    }
}

impl FuzzySets {
    pub fn contains(&self, set: &str) -> bool {
        let mut contains = false;
        contains = self.start.name == set || self.end.name == set;
        for triangle in &self.triangles {
            if triangle.name == set {
                contains = true;
                break;
            }
        }
        return contains;
    }
    pub fn fuzzify_input(&self, input: f64) -> FuzzySetResult {
        let mut result = FuzzySetResult {
            items: Vec::new(),
        };
        let start = self.start.fuzzify(input);

        if start > 0.0 {
            result.items.push((self.start.name.clone(), start))
        }
        for triangle in &self.triangles {
            let val = triangle.fuzzify(input);
            if val > 0.0 {
                result.items.push((triangle.name.clone(), val))
            }
        }
        let end = self.end.fuzzify(input);
        if end > 0.0 {
            result.items.push((self.end.name.clone(), end))
        }
        return result;
    }

    pub fn cog(&self, increment: f64) -> f64 {
        let mut top: f64 = 0.0;
        let mut bot: f64 = 0.0;

        if self.start.clip != 0.0 {
            let mut i = self.start.x0 * 10.0;
            top += (i / 10.0) * self.start.fuzzify(i / 10.0);
            bot += self.start.fuzzify(i / 10.0);
            while i != self.start.x1 * 10.0 {
                i += increment * 10.0;
                top += (i / 10.0) * self.start.fuzzify(i / 10.0);
                bot += self.start.fuzzify(i / 10.0);
            }
        }
        for triangle in self.triangles.iter() {
            if triangle.clip != 0.0 {
                let mut i = triangle.x0 * 10.0;
                top += (i / 10.0) * triangle.fuzzify(i / 10.0);
                bot += triangle.fuzzify(i / 10.0);
                while i != triangle.x2 * 10.0 {
                    i += increment * 10.0;
                    top += (i / 10.0) * triangle.fuzzify(i / 10.0);
                    bot += triangle.fuzzify(i / 10.0);
                }
            }
        }
        if self.end.clip != 0.0 {
            let mut i = self.end.x0 * 10.0;

            top += (i / 10.0) * self.end.fuzzify(i / 10.0);
            bot += self.end.fuzzify(i / 10.0);
            while i != self.end.x1 * 10.0 {
                i += increment * 10.0;
                top += (i / 10.0) * self.end.fuzzify(i / 10.0);
                bot += self.end.fuzzify(i / 10.0);
            }
        }
        return top / bot;
    }
}
