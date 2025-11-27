use crate::microformat_extractor;
use crate::types::HGeo;

microformat_extractor! {
    HGeo, ".h-geo" {
        latitude: f64_number(".p-latitude"),
        longitude: f64_number(".p-longitude"),
        altitude: f64_number(".p-altitude"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hgeo() {
        let html = r#"
            <div class="h-geo">
                <span class="p-latitude">37.7749</span>
                <span class="p-longitude">-122.4194</span>
            </div>
        "#;

        let geos = extract(html, None).unwrap();
        assert_eq!(geos.len(), 1);
        assert!((geos[0].latitude.unwrap() - 37.7749).abs() < 0.0001);
        assert!((geos[0].longitude.unwrap() - (-122.4194)).abs() < 0.0001);
    }

    #[test]
    fn test_hgeo_with_altitude() {
        let html = r#"
            <div class="h-geo">
                <span class="p-latitude">40.7128</span>
                <span class="p-longitude">-74.0060</span>
                <span class="p-altitude">10</span>
            </div>
        "#;

        let geos = extract(html, None).unwrap();
        assert_eq!(geos.len(), 1);
        assert!((geos[0].latitude.unwrap() - 40.7128).abs() < 0.0001);
        assert!((geos[0].longitude.unwrap() - (-74.0060)).abs() < 0.0001);
        assert!((geos[0].altitude.unwrap() - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_hgeo_negative_altitude() {
        let html = r#"
            <div class="h-geo">
                <span class="p-latitude">31.5590</span>
                <span class="p-longitude">35.4732</span>
                <span class="p-altitude">-430.5</span>
            </div>
        "#;

        let geos = extract(html, None).unwrap();
        assert_eq!(geos.len(), 1);
        assert!((geos[0].altitude.unwrap() - (-430.5)).abs() < 0.1);
    }

    #[test]
    fn test_multiple_hgeo() {
        let html = r#"
            <div class="h-geo">
                <span class="p-latitude">51.5074</span>
                <span class="p-longitude">-0.1278</span>
            </div>
            <div class="h-geo">
                <span class="p-latitude">48.8566</span>
                <span class="p-longitude">2.3522</span>
            </div>
        "#;

        let geos = extract(html, None).unwrap();
        assert_eq!(geos.len(), 2);
        assert!((geos[0].latitude.unwrap() - 51.5074).abs() < 0.0001);
        assert!((geos[1].latitude.unwrap() - 48.8566).abs() < 0.0001);
    }
}
