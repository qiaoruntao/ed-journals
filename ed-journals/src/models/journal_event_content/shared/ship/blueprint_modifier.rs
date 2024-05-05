use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum BlueprintModifier {
    Size,
    Class,
    Mass,
    Integrity,
    PowerDraw,
    BootTime,
    ShieldBankSpinUp,
    ShieldBankDuration,
    ShieldBankReinforcement,
    ShieldBankHeat,
    DamagePerSecond,
    Damage,
    DistributorDraw,
    ThermalLoad,
    ArmourPenetration,
    MaximumRange,
    ShotSpeed,
    RateOfFire,
    BurstRateOfFire,
    BurstSize,
    AmmoClipSize,
    AmmoMaximum,
    RoundsPerShot,
    ReloadTime,
    BreachDamage,
    MinBreachChance,
    MaxBreachChance,
    Jitter,
    WeaponMode,
    DamageType,
    ShieldGenMinimumMass,
    ShieldGenOptimalMass,
    ShieldGenMaximumMass,
    ShieldGenMinStrength,
    ShieldGenStrength,
    ShieldGenMaxStrength,
    RegenRate,
    BrokenRegenRate,
    EnergyPerRegen,
    FSDOptimalMass,
    FSDHeatRate,
    MaxFuelPerJump,
    EngineMinimumMass,
    EngineOptimalMass,
    MaximumMass,
    EngineMinPerformance,
    EngineOptPerformance,
    EngineMaxPerformance,
    EngineHeatRate,
    PowerCapacity,
    HeatEfficiency,
    WeaponsCapacity,
    WeaponsRecharge,
    EnginesCapacity,
    EnginesRecharge,
    SystemsCapacity,
    SystemsRecharge,
    DefenceModifierHealthMultiplier,
    DefenceModifierHealthAddition,
    DefenceModifierShieldMultiplier,
    DefenceModifierShieldAddition,
    KineticResistance,
    ThermicResistance,
    ExplosiveResistance,
    CausticResistance,
    FSDInterdictorRange,
    FSDInterdictorFacingLimit,
    ScannerRange,
    DiscoveryScannerRange,
    DiscoveryScannerPassiveRange,
    MaxAngle,
    ScannerTimeToScan,
    ChaffJamDuration,
    ECMRange,
    ECMTimeToCharge,
    ECMActivePowerConsumption,
    ECMHeat,
    ECMCooldown,
    HeatSinkDuration,
    ThermalDrain,
    NumBuggySlots,
    CargoCapacity,
    MaxActiveDrones,
    DroneTargetRange,
    DroneLifeTime,
    DroneSpeed,
    DroneMultiTargetSpeed,
    DroneFuelCapacity,
    DroneRepairCapacity,
    DroneHackingTime,
    DroneMinJettisonedCargo,
    DroneMaxJettisonedCargo,
    FuelScoopRate,
    FuelCapacity,
    OxygenTimeCapacity,
    RefineryBins,
    AFMRepairCapacity,
    AFMRepairConsumption,
    AFMRepairPerAmmo,
    MaxRange,
    SensorTargetScanAngle,
    Range,
    VehicleCargoCapacity,
    VehicleHullMass,
    VehicleFuelCapacity,
    VehicleArmourHealth,
    VehicleShieldHealth,
    FighterMaxSpeed,
    FighterBoostSpeed,
    FighterPitchRate,
    FighterDPS,
    FighterYawRate,
    FighterRollRate,
    CabinCapacity,
    CabinClass,
    DisruptionBarrierRange,
    DisruptionBarrierChargeDuration,
    DisruptionBarrierActivePower,
    DisruptionBarrierCooldown,
    WingDamageReduction,
    WingMinDuration,
    WingMaxDuration,
    ShieldSacrificeAmountRemoved,
    ShieldSacrificeAmountGiven,
    FSDJumpRangeBoost,
    FSDFuelUseIncrease,
    BoostSpeedMultiplier,
    BoostAugmenterPowerUse,
    ModuleDefenceAbsorption,
    FalloffRange,
    DSS_RangeMult,
    DSS_AngleMult,
    DSS_RateMult,
    DamageFalloffRange,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
