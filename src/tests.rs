use crate::daemon::{FanCurve, FanPoint, PowerProfile};

#[test]
fn test_power_profile_cycle() {
    let mut profile = PowerProfile::Quiet;
    
    profile = profile.cycle_next();
    assert_eq!(profile, PowerProfile::Balanced);
    
    profile = profile.cycle_next();
    assert_eq!(profile, PowerProfile::Performance);
    
    profile = profile.cycle_next();
    assert_eq!(profile, PowerProfile::Quiet);
}

#[test]
fn test_power_profile_conversion() {
    assert_eq!(PowerProfile::from_u32(0), PowerProfile::Balanced);
    assert_eq!(PowerProfile::from_u32(1), PowerProfile::Performance);
    assert_eq!(PowerProfile::from_u32(3), PowerProfile::Quiet);
    assert_eq!(PowerProfile::from_u32(99), PowerProfile::Balanced); // Fallback

    assert_eq!(PowerProfile::Balanced.to_u32(), 0);
    assert_eq!(PowerProfile::Performance.to_u32(), 1);
    assert_eq!(PowerProfile::Quiet.to_u32(), 3);
}

#[test]
fn test_fan_curve_default() {
    let curve = FanCurve::default_curve();
    assert!(!curve.enabled);
    assert_eq!(curve.cpu_curve.len(), 8);
    assert_eq!(curve.gpu_curve.len(), 8);
    
    // Check first and last points
    assert_eq!(curve.cpu_curve[0], FanPoint { temp: 30, speed: 0 });
    assert_eq!(curve.cpu_curve[7], FanPoint { temp: 100, speed: 100 });
}

#[test]
fn test_fan_point_validity() {
    let point = FanPoint { temp: 30, speed: 0 };
    // Just ensuring type structure is correct, deeper logic validation 
    // depends on where we enforce limits (currently primarily UI)
    assert_eq!(point.temp, 30);
    assert_eq!(point.speed, 0);
}
