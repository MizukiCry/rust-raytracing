use crate::*;

pub fn earth() -> (Camera, HittableList) {
    let mut camera = Camera::default();
    camera.camera_center = Vec3::new(0.0, 0.0, 12.0);
    camera.initialize();

    let mut world = HittableList::default();

    let erath_texture = Rc::new(ImageTexture::from("img/earthmap.jpg"));
    let earth_material = Rc::new(Lambertian::new(Rc::clone(&erath_texture) as Rc<dyn Texture>));
    world.add(Rc::new(Sphere::new(
        Vec3::default(),
        2.0,
        Rc::clone(&earth_material) as Rc<dyn Material>,
    )));

    (camera, world)
}
