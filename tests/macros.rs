#[macro_use]
extern crate log_ndc;

#[test]
fn all_macors() {
    ndc_trace!("tr");
    ndc_trace!("tr {}", 1);
    ndc_trace!(target: "foobar", "tr {}", 1);

    ndc_debug!("tr");
    ndc_debug!("tr {}", 1);
    ndc_debug!(target: "foobar", "tr {}", 1);

    ndc_info!("tr");
    ndc_info!("tr {}", 1);
    ndc_info!(target: "foobar", "tr {}", 1);

    ndc_warn!("tr");
    ndc_warn!("tr {}", 1);
    ndc_warn!(target: "foobar", "tr {}", 1);

    ndc_error!("tr");
    ndc_error!("tr {}", 1);
    ndc_error!(target: "foobar", "tr {}", 1);
}
