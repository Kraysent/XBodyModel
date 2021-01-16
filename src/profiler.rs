use std::time::Instant;

pub struct Profiler
{
    pub t: Instant,
    pub name: &'static str
}

impl Profiler
{
    pub fn new(name: Option<&'static str>) -> Profiler
    {
        return Profiler { t: Instant::now(), name: name.unwrap_or("") };
    }
}

impl Drop for Profiler
{
    fn drop(&mut self) 
    { 
        eprintln!("({}): {} ms", self.name, self.t.elapsed().as_millis());
    }
}