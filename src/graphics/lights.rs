use crate::graphics::{vector::Vec3, RGB};
/// Represents lighting configuration
#[derive(Copy, Clone, Debug)]
pub struct LightConfig {
    /// view: view vector
    pub view: Vec3,
    /// ambient: color of ambient light
    pub ambient_color: RGB,
    /// color of directional light source
    pub dir_color: RGB,
    /// location of directional light source
    pub dir_vec: Vec3,
    /// ambient reflection const
    pub areflect: Vec3,
    /// diffuse reflection const
    pub dreflect: Vec3,
    /// specular reflection const
    pub sreflect: Vec3,
}

impl LightConfig {
    pub fn get_color_from_norm(&self, normal: Vec3) -> RGB {
        let normaln = normal.norm();
        let viewn = self.view.norm();
        let dirvecn = self.dir_vec.norm();

        let ndotdir: f64 = normaln.dot(dirvecn).max(0.);

        let iambient: Vec3 = self.areflect.mul_across(Vec3::from(self.ambient_color));
        let idiffuse: Vec3 = Vec3::from(self.dir_color).mul_across(self.dreflect) * ndotdir;
        let ispecular: Vec3 = Vec3::from(self.dir_color).mul_across(self.sreflect)
            * (((2 * normaln * ndotdir - dirvecn) * viewn).max(0.).powi(10));

        (iambient.limit(0., 255.) + idiffuse.limit(0., 255.) + ispecular.limit(0., 255.)).into()
    }

    pub const TEST_LIGHT: Self = Self {
        view: Vec3(0., 0., 1.),
        ambient_color: RGB {
            red: 0,
            green: 0,
            blue: 0,
        },
        dir_color: RGB {
            red: 252,
            green: 219,
            blue: 3,
        },
        dir_vec: Vec3(0.5, 0.75, 1.),
        areflect: Vec3(0.1, 0.1, 0.1),
        dreflect: Vec3(0.5, 0.5, 0.5),
        sreflect: Vec3(0.5, 0.5, 0.5),
    };
}
