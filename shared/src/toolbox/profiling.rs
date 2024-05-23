use pyroscope::PyroscopeAgent;
use pyroscope_pprofrs::{pprof_backend, PprofConfig};

pub fn start(service_name: &str) -> anyhow::Result<()> {
  let user = std::env::var("PYROSCOPE_USER").unwrap();
  let password = std::env::var("PYROSCOPE_TOKEN").unwrap();
  let agent = PyroscopeAgent::builder("https://profiles-prod-002.grafana.net", service_name)
    .basic_auth(user, password)
    .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
    .tags([("TagA", "ValueA"), ("TagB", "ValueB")].to_vec())
    .build()?;
  let _agent_running = agent.start()?;

  Ok(())
}
