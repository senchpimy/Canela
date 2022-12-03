mod mensajes;

use mensajes::{enviar,obtener_mensajes};
use eframe::{egui::{CentralPanel,ScrollArea,Separator,TopBottomPanel,SidePanel,Context}, epi::App, run_native, NativeOptions};
use egui_extras::RetainedImage;
use egui;


const APP_NAME: &str = "MENSAJES";

fn main() {
    tracing_subscriber::fmt::init();

    let options = NativeOptions::default();
    let app =MyApp::new();

    run_native(
        Box::new(app),
        options,
    );
}


struct MyApp {
    name:String,
    contactos:Vec<Usuario>,
    configuration:bool,
    selected_user:String,
    profile_pic:RetainedImage,
    mensaje:String,
}

struct Usuario{
    username:String,
    profile_pic:String,
    //id:i32
}


impl MyApp {
    fn new() -> MyApp {
        Self {
            name: "Arthur".to_owned(),
            contactos: vec![Usuario{
            username:String::from("lalo"),
            profile_pic:String::from("foto kawai"),
            },
            Usuario{
            username:String::from("lalo1"),
            profile_pic:String::from("foto kawai2"),
            },
            Usuario{
            username:String::from("lalo2"),
            profile_pic:String::from("foto kawai3"),
            },
            ],
            configuration:true,
            selected_user:"bienbenido".to_owned(),
            profile_pic: RetainedImage::from_image_bytes("./image.png",include_bytes!("./image.png")).unwrap(),
            mensaje: "".to_owned(),
        }
    }
}

////////////////////////////////////////////////////////////////////
impl App for MyApp {
    fn setup(
        &mut self,
        ctx: &Context,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        //self.configure_fonts(ctx);
        println!("lalal");
    }

    fn update(&mut self, ctx:  &Context, _frame: &eframe::epi::Frame) {
    //println!("{}",frame.info().name);
    if self.configuration{
        TopBottomPanel::top("configuraciones").show(ctx, |ui|{
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
            let image_button = egui::ImageButton::new(self.profile_pic.texture_id(ctx),[30.,30.]);
            ui.label(self.name.to_string());
        if ui.add(image_button).clicked(){
            self.configuration=false;
        }
        ui.label("                                 ");
        ui.label(self.selected_user.to_string());
        });
            });
        contactos_configuraciones(ctx, &mut self.contactos,&mut self.selected_user);
        barra_de_mensajes(ctx, &mut self.mensaje);
        render_mensajes(ctx);

    }else{
        configuraciones(ctx,&mut self.configuration,&mut self.profile_pic);
    }

    }

    fn name(&self)->&str{
        &APP_NAME
    }
}
////////////////////////////////////////////////////////////////////

fn contactos_configuraciones(ctx:&Context, contactos: &mut Vec<Usuario>, titulo:&mut String){
    SidePanel::left("my_side_panel").show(ctx,|ui| {
        ui.label("lalal aqui van los contactos y grupos");

        ScrollArea::vertical().show(ui, |ui| {
            for contacto in contactos {
                ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    if ui.button(contacto.username.to_string()).clicked(){
                        *titulo=String::from(contacto.profile_pic.to_string());
                    }
                });
                });
            ui.add(Separator::default().spacing(3.).horizontal());
            }
        });
    });
}


fn barra_de_mensajes(ctx:&Context, texto:&mut String){
    TopBottomPanel::bottom("lalalaal aqui valo de escribir mensajes").show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| ui.text_edit_multiline(texto));
        enviar(texto);
    });

}

fn render_mensajes(ctx:&Context){
    CentralPanel::default().show(ctx, |ui| {
        ScrollArea::vertical().show(ui, |ui| {
            for element in obtener_mensajes(){
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                        if ui.label("papu").clicked() {
                        println!("{}",element.usuario);
                        }
                         ui.label(element.contenido);
                         ui.add(Separator::default().spacing(3.).horizontal());
                    });
            }
        });
    });
}

fn configuraciones(ctx:&Context, regresobool:&mut bool,imagen:&mut RetainedImage){
        CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(imagen.texture_id(ctx),[200.,200.]));
            ui.label("lalalala aqui va la configuracion");
            egui::widgets::global_dark_light_mode_switch(ui);
            if ui.button("lala").clicked(){
                *regresobool = true;
            }
        });
    });
}
