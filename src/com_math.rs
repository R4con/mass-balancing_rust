#[derive(Clone,Debug)]
pub struct PointObj {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub Mass: f32
}

impl PointObj {
    pub fn calculate_centerofmass(ObjList: &Vec<PointObj> ) -> PointObj {
        let mut CenterofMass = PointObj {x: 0.0, y: 0.0, z: 0.0, Mass: 0.0};

        for Obj in ObjList.iter() {
            CenterofMass.Mass += Obj.Mass;
            CenterofMass.x += Obj.Mass * Obj.x;
            CenterofMass.y += Obj.Mass * Obj.y;
            CenterofMass.z += Obj.Mass * Obj.z;
        }

        CenterofMass.x = CenterofMass.x / CenterofMass.Mass;
        CenterofMass.y = CenterofMass.y / CenterofMass.Mass;
        CenterofMass.z = CenterofMass.z / CenterofMass.Mass;

        CenterofMass
    }

    pub fn calculate_distance(&self, PointB: &PointObj) -> f32 {
        ( ( (self.x - PointB.x).powf(2.0) + (self.y - PointB.y).powf(2.0) + (self.z - PointB.z).powf(2.0) ).sqrt() ).abs()
    }
}