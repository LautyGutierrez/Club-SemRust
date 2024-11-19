//segundo contrato
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reporte {
    use club::ClubRef;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use ink_prelude::string::ToString;
  	

    ///En el struct Reporte se va guardar el contrato del Club
    #[ink(storage)]
    pub struct Reporte {
        #[cfg(not(test))]
        club: ClubRef,//solo compila cuando no se estan corriendo los test
    }
    ///El struct Fecha nos permite leer una fecha legible en dia, mes y año
    #[derive(scale::Decode, scale::Encode,Clone,Copy,Debug,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Fecha{
        dia:u64,
        mes:u64,
        anio:u64,
    }
  	///El struct Socio es para mockear los socios y realizar los tests
    struct Socio {
      dni: u128,
      categoria: String,
      actividad: String,
  	}
  	///El struct Pago es para mockear los pagos y realizar los tests
  	struct Pago {
      dni_socio:u128,
      id: u128,
      fecha: u64,
      pagado: bool,
      costo: u128,
  	}
  	///El struct Info es para mockear el club y realizar los tests, este va a ser el que provee la informacion
  	struct Info{
      #[cfg(test)]
      socios:Vec<Socio>,
      #[cfg(test)]
      pagos:Vec<Pago>
      
  	}
  	impl Socio{
        ///Crea un socio y lo retorna,recibe el dni, la categoria y la actividad del socio
        /// Ejemplo
        /// '''
        ///     self.crear_socio(44851840,"A".to_string(),"TODO".to_string())
        /// '''
        fn crear_socio(dni: u128, categoria: String, actividad: String)->Self{
            Self{
            dni,
            categoria,
            actividad,
            }
        }
    }
    impl Pago{
        ///Crea un pago y lo retorna,recibe el dni, el id,la fecha en la que se esta realizado, si fue pagado o no y el costo
        /// Ejemplo
        ///'''
        ///     self.crear_pago(44851840,2,227788993900,true,5000)
        ///'''
        fn crear_pago(dni_socio:u128, id: u128,fecha: u64,pagado: bool,costo: u128)->Self{
            Self{
                dni_socio,
                id,
                fecha,
                pagado,
                costo,
            }
        }
    }
    impl Reporte{
        ///Genera un nuevo contrato, se le pasa como parametro la referencia al cotrato Club y la guarda.

        #[ink(constructor)]
        #[cfg(not(test))]
        pub fn new(club: ClubRef)-> Self{
            Self{
                club
            }
            
        }
        ///Metodo mockeado para el test
        #[cfg(test)]
        pub fn new()->Self{
            Self { }
        }
        ///Recibe del contrato Club el listado de los socios y los devuelve
        #[cfg(not(test))]
        fn get_socios(&self)->Vec<u128>{
            self.club.get_socios()
        }
        ///Recibe del contrato Club el listado de los pagos de un socio en especifico y los devuelve
        #[cfg(not(test))]
        fn get_pagos(&self, dni: u128) -> Vec<(u128, u64 ,bool, u128)> {
           let v= self.club.get_pago(dni);
           if v.len() == 0{
            panic!("no existe el socio");
           }else{
            return v;
           }
        }
        ///Nos devuelve la fecha de hoy 
        #[cfg(not(test))]
        pub fn tiempo(&self)->Timestamp{
            self.env().block_timestamp()

        }
        ///Metodo mockeado para la fecha 
        #[cfg(test)]
        pub fn tiempo(&self)->Timestamp{
            1689711702000
        }
        ///Recibe del contrato Club la informacion de un socio en especifico y la devuelve
        #[cfg(not(test))]
        fn get_info_socio(&self, pos:u128)->(String,String){
            self.club.get_info_socio(pos)
        }
        ///Carga la informacion necesaria para mockear el club y poder probar los test 
        #[cfg(test)]
        fn crear_info(&self)-> Info{
            let socios: Vec<Socio> = Vec::new();
            let pagos: Vec<Pago> = Vec::new();
            let mut i = Info{
              socios,
              pagos,
            };
            i.socios.push(Socio::crear_socio(44851840, "A".to_string(), "TODOS".to_string()));
            i.socios.push(Socio::crear_socio(44851841, "B".to_string(), "FUTBOL".to_string()));
            i.socios.push(Socio::crear_socio(44851842, "C".to_string(), "NADA".to_string()));
            i.socios.push(Socio::crear_socio(44851843, "A".to_string(), "TODOS".to_string()));
            i.socios.push(Socio::crear_socio(44851844, "B".to_string(), "TENIS".to_string()));
            i.socios.push(Socio::crear_socio(44851845, "C".to_string(), "NADA".to_string()));
                
            i.pagos.push(Pago::crear_pago(44851840, 1, 1, true, 5000));
            i.pagos.push(Pago::crear_pago(44851840, 2, self.tiempo(), true, 5000));
            i.pagos.push(Pago::crear_pago(44851841, 3, self.tiempo(), true, 3000));
            i.pagos.push(Pago::crear_pago(44851841, 4, self.tiempo(), true, 3000));
            i.pagos.push(Pago::crear_pago(44851842, 5, self.tiempo(), true, 2000));
            i.pagos.push(Pago::crear_pago(44851842, 6, self.tiempo(), true, 2000));
            i.pagos.push(Pago::crear_pago(44851843, 7, 1, false, 5000));
            i.pagos.push(Pago::crear_pago(44851843, 8, self.tiempo(), true, 5000));
            i.pagos.push(Pago::crear_pago(44851844, 9, self.tiempo(), true, 3000));
            i.pagos.push(Pago::crear_pago(44851844, 10, 1, false, 3000));
            i.pagos.push(Pago::crear_pago(44851845, 11, self.tiempo(), true, 2000));
            i.pagos.push(Pago::crear_pago(44851845, 12, 1, false, 2000));
            return i;
        }
        /// metodo mockeado para el testing de socios,devuelve un listado de los dni de los socios
        #[cfg(test)]
        fn get_socios(&self)->Vec<u128>{
            let info = self.crear_info();
            let mut vec: Vec<u128>=Vec::new();
            for i in info.socios{
                vec.push(i.dni);
            }
            vec
        }

        ///metodo mockeado para el testing de pagos, devuelve un listado de los pagos de un socio en especifico
        #[cfg(test)]
        fn get_pagos(&self,dni: u128)->Vec<(u128, u64 ,bool, u128)>{
            let info = self.crear_info();
          	let vec = info.pagos
            .iter()
            .filter(|pago| pago.dni_socio == dni)
            .map(|pago| (pago.id, pago.fecha, pago.pagado, pago.costo))
            .collect();
          	vec
        }
        ///Metodo mockeado para el testing, devuelva la categoria y la actividad de un socio en especifico
      	#[cfg(test)]
      	fn get_info_socio(&self, i: u128)->(String, String){
          let info = self.crear_info();
          let tupla = (info.socios[i as usize].categoria.clone(), info.socios[i as usize].actividad.clone());
          tupla
        }
        ///Crea un vector con los DNIs de los socios morosos y los retorna
        ///Obtiene un vector con los DNIs de los socios, y si es moroso los agrega al vector
        ///Si ninguno es moroso devuelve un vector vacio
        #[ink(message)]
        pub fn get_pagos_pendientes(&self)->Vec<u128>{
            let mut vector:Vec<u128> = Vec::new();
            let socios = self.get_socios();
            let fecha_hoy:u64 = self.tiempo();
            for socio in socios{
                if self.es_moroso(socio,fecha_hoy){
                    vector.push(socio);
                }
            }
            vector
        }

        /// es moroso devulve un booleano, indicando si el socio es moroso o no lo es
        ///llama a la funcion 'get_pago' que devuelve un vector de tuplas con informacion de los pagos de un socio en especifico
        ///con esa informacion se fija si pago o no pago, y si no pago, se fija si se paso de la fecha de vencimiento indicando que es moroso
        
        fn es_moroso(&self, socio: u128,fecha_hoy:u64)->bool{
            let pagos = self.get_pagos(socio);
            let mut ok = false;
            let mut i = 0;
            while i < pagos.len() && !ok{
                if !pagos[i].2{ 
                    if fecha_hoy > pagos[i].1{
                        ok = true;
                    }
                }
                i+=1;

            }
            return ok;
        }
        ///Nos devuelve un vector con los socios no morosos que tienen permitido asistir a una actividad deportiva especifica
        /// La actividad la recibe como parametro, llama a la funcion 'get_socios' para obtener un listado de los dni de los socios
        /// Llama a otra funcion para obtener la informacion de la categoria y actividad de ese socio en especifico 
        /// Si no es moroso ese socio, nos fijamos su categoria
        /// 	Si tiene categoria 'A', como tiene permitido todas las actividades, lo agrego al listado
        ///		Si es categoria 'B', me fijo si la actividad es igual a la que le pasaron por parametro y si es igual, se agrega
        ///		Si es categoria 'C', como no tiene permitido ninguna actividad extra, no se incluye directamente
        ///Si es moroso, no se agrega al listado.
      	///Si ninguno cumple los requerimientos, devuelve un vector vacio
        #[ink(message)]
        pub fn get_socios_no_morosos_actividad_especifica(&self, actividad: String)->Vec<u128>{
            let mut vector: Vec<u128> = Vec::new();
            if self.es_actividad_valida(&actividad){ 
                let socios = self.get_socios();
                let fecha_hoy = self.tiempo();
                for i in 0..socios.len(){
                    let info = self.get_info_socio(i as u128); //falta ver esto
                    if !self.es_moroso(socios[i], fecha_hoy) {
                        if info.0 == "A"{
                            vector.push(socios[i]);
                        }else{
                            if info.0 == "B"{
                                if info.1 == actividad{
                                    vector.push(socios[i]);
                                }
                            }
                        }
                    }
                } 
            }
            return vector;
        }
        ///Recibe un actividad y retorna true si es una actividad del club, false en caso contrario
        fn es_actividad_valida(&self,actividad: &String)->bool{
            let act = match actividad as &str{
                "FUTBOL"=>true, 
                "BASQUET"=>true, 
                "RUGBY"=>true,
                "HOCKEY"=>true,
                "NATACION"=>true, 
                "TENIS"=>true,
                "PADDLE"=>true,
                _=>panic!("actividad incorrecta"),
            };
            act
        }
        /// calcular_fecha recibe una fecha (timestamp) y lo devuelve en formato fecha
        /// Sacamos la cantidad de dias que le tenemos que sumar y se la pasamos a la funcion 'sumar_dias' para que nos devuelva la fecha sumada con esa cantidad de dias y la retornamos
    
        fn calcular_fecha(&self, time:u64)->Fecha{
            let dia = 1;
            let mes = 1;
            let anio = 1970;
            let mut f = Fecha{
                dia,
                mes,
                anio,
            };
            //como en ink el timestamp es en milisegundos lo paso a segundos 
            let i=time.checked_div(1000);
            if let Some(p)= i{
                //saco la cantidad de dias
                let cant=p.checked_div(86400);
                if let Some(c)=cant{
                    f.sumar_dias(c);
                }
            
            }
            return f;
        }
        ///Recibe un mes y un año y retorna un listado con la recaudacion de cada categoria de ese mes y año    
        #[ink(message)]
        pub fn recaudacion_mensual(&self, mes: u64, anio: u64)->Vec<(String, u128)>{
            let mut map:Vec<(String,u128)> = Vec::new();
            let mut a = 0;
            let mut b = 0;
            let mut c = 0;
            let socios = self.get_socios();
            for i in 0..socios.len(){
                let pagos = self.get_pagos(socios[i]);
                let info = self.get_info_socio(i as u128); //falta esto
                for j in 0..pagos.len(){
                    let fecha = self.calcular_fecha(pagos[j].1);
                    if fecha.mes == mes && fecha.anio == anio{
                        match &info.0 as &str {
                            "A"=> a+=pagos[j].3,
                            "B"=> b+=pagos[j].3,
                            "C"=> c+=pagos[j].3,
                            _=>(),
                        }
                    }
                }
            }
            map.push(("A".to_string(), a));
            map.push(("B".to_string(), b));
            map.push(("C".to_string(), c));
            return map;
        }
    }
    impl Fecha{
        ///esta funcion nos dice si el anio es bisiesto o no
        /// Nos devuelve verdadero si es bisiesto, falso si no lo es
        fn es_bisiesto(&self)->bool{
            let mut ok = true;
            if self.anio % 4 == 0 {
                if self.anio % 100 == 0 && !self.anio % 400 == 0{
                    ok=false;
                }

            } else {
                ok = false;
            }
            return ok;
        }
        ///A la fecha que esta guardada, le suma la cantidad de dias pasado por parametro
        fn sumar_dias(&mut self,mut cant:u64){
            while cant > 0{
                let d:u64;
                match self.mes {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => d = 31,
                    4 | 6 | 9 | 11 => d = 30,
                    2 => if self.es_bisiesto() {d = 29 } else { d = 28 },
                    _ => d=0,
                }
                if self.dia + cant <= d   {
                    self.dia += cant;
                    cant -= cant;
                } else {
                    cant = cant - (d - self.dia);
                    self.dia = 0;
                    self.mes += 1;
                    if self.mes == 13{
                        self.anio += 1;
                        self.mes = 1;
                    }
                }
            }
        }
    }
   
    #[cfg(test)]
	mod tests {
        use super::*;
        
        #[ink::test]
        fn crear_info_test() {
          let reporte = Reporte::new();
          let info = reporte.crear_info();
          assert_eq!(info.socios.len(), 6);
          assert_eq!(info.pagos.len(), 12);
        }
    	#[ink::test]
        fn get_socios_test() {
          let reporte = Reporte::new();
          let socios = reporte.get_socios();
          assert_eq!(socios.len(), 6);
          assert_eq!(socios[0], 44851840);
        }
        #[ink::test]
    	fn get_pagos_test() {
          let reporte = Reporte::new();
          let pagos = reporte.get_pagos(44851840);
          assert_eq!(pagos.len(), 2);
          assert_eq!(pagos[0].1, 1);
          assert!(pagos[0].2);
        }
        #[ink::test]
    	fn get_info_socio_test() {
          let reporte = Reporte::new();
          let info = reporte.get_info_socio(0);
          assert_eq!(info.0, "A");
          assert_eq!(info.1, "TODOS");
        }
    	#[ink::test]
        fn es_moroso_test(){
          let reporte = Reporte::new();
          let ok1 = reporte.es_moroso(44851840, 12345678);
          let ok2 = reporte.es_moroso(44851845, 12345678);
          assert!(!ok1);
          assert!(ok2);
        }
        #[ink::test]
        fn get_pagos_pendientes_test(){
          let reporte = Reporte::new();
          let vector = reporte.get_pagos_pendientes();
          assert_eq!(vector.len(), 3);
        }
        #[ink::test]
        fn calcular_fecha_test(){
            let r = Reporte::new();
            let a=r.calcular_fecha(1689202523000);
            assert_eq!(a.mes,7);
            assert_eq!(a.anio,2023);
        }
        #[ink::test]
        fn es_bisiesto_test(){
            let dia = 1;
            let mes = 2;
            let anio = 2000;
            let f = Fecha{
                dia,
                mes,
                anio,
            };
            assert!(f.es_bisiesto(),"No es bisiesto!");
        }
        #[ink::test]
    	fn no_es_bisiesto_test(){
            let dia = 1;
            let mes = 2;
            let anio = 2001;
            let f = Fecha{
                dia,
                mes,
                anio,
            };
            assert!(!f.es_bisiesto(),"Es bisiesto!");
        }
        #[ink::test]
        fn sumar_dias_test(){
            let dia = 1;
            let mes = 2;
            let anio = 2000;
            let mut f = Fecha{
                dia,
                mes,
                anio,
            };
            f.sumar_dias(32);
            assert_eq!(f.mes,3);
            assert_eq!(f.anio,2000);
        }
        #[ink::test]
        fn recaudacion_mensual_test(){
            let r = Reporte::new();
            let vec=r.recaudacion_mensual(7,2023);
            assert_eq!(vec[0].0,"A");
            assert_eq!(vec[0].1,10000);
          	assert_eq!(vec[1].0,"B");
            assert_eq!(vec[1].1,9000);
          	assert_eq!(vec[2].0,"C");
            assert_eq!(vec[2].1,6000);
        }
        
        #[ink::test]
        fn get_socios_no_morosos_actividad_especifica_test(){
            let r = Reporte::new();
            let v = r.get_socios_no_morosos_actividad_especifica("FUTBOL".to_string());
            assert_eq!(v[0], 44851840);
            assert_eq!(v[1], 44851841);
  	    }
        #[ink::test]
        #[should_panic(expected = "actividad incorrecta")]
          fn es_actividad_valida_test(){
                let r = Reporte::new();
                assert!(r.es_actividad_valida(&"FUTBOL".to_string()));
                assert!(r.es_actividad_valida(&"BASQUET".to_string()));
                assert!(r.es_actividad_valida(&"HOCKEY".to_string()));
                assert!(r.es_actividad_valida(&"PADDLE".to_string()));
                assert!(r.es_actividad_valida(&"TENIS".to_string()));
                assert!(r.es_actividad_valida(&"RUGBY".to_string()));
                assert!(r.es_actividad_valida(&"NATACION".to_string()));
                assert!(!r.es_actividad_valida(&"ATLETISMO".to_string()));
            }
    }
  
}
