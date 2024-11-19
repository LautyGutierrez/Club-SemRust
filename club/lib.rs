
#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::club::ClubRef;

#[ink::contract]
mod club{
  use core::panic;
  use ink::storage::Mapping;
  use ink::prelude::vec::Vec;
  use ink::prelude::string::String;
  use ink_prelude::string::ToString;

  ///En el struct de Club se va a guardar:
  ///-Informacion de todos los socios que se registren en el club
  ///-Los pagos de cada socio registrado
  ///-Precio de las tres categorias
  ///-El owner guarda la direccion del duenio del contrato
  ///-El vector de direcciones guarda todas las direcciones autorizadas para realizar operaciones
  ///-La politica nos indica que si esta desactivada, cualquiera podra realizar operaciones, y si esta activada, solo las direcciones autorizadas podran hacerlo
  ///-El descuento que se le otorga al socio el proximo mes si llega a tener la cantidad correspondiente de pagos consecutivos sin atrasos
  ///-La cantidad de meses que el socio debe acumular de pagos consecutivos sin atrasos para obtener un descuento
  
  #[ink(storage)]
	pub struct Club {
  	socios: Vec<Socio>,
  	pagos:Vec<Pago>,
    precio_categoria: Mapping<TipoCategoria, u128>,
    owner: Option<AccountId>,
    direcciones: Vec<AccountId>,
    politica: bool,
    descuento: u128,
    cantidad_meses: u128,
  }
  #[derive(scale::Decode, scale::Encode,Debug,PartialEq)]
  #[cfg_attr(
      feature = "std",
      derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
  )]
  enum Actividad{
  	FUTBOL, 
    BASQUET, 
    RUGBY, 
    HOCKEY, 
    NATACION, 
  	TENIS,
    PADDLE,
    TODOS,
  }
  #[derive(scale::Decode, scale::Encode,Clone,Copy,PartialEq,Debug)]
  #[cfg_attr(
      feature = "std",
      derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
  )]
  enum TipoCategoria{
  	A,
    B,
    C,
  }
  #[derive(scale::Decode, scale::Encode,Debug,PartialEq)]
  #[cfg_attr(
      feature = "std",
      derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
  )]
  ///En el Struct Pago se van a guardar los datos de los pagos de los socios
  ///De cada pago se conoce su id(que es la posicion que ocupa en el vector),el dni del socio,
  ///el costo del pago, la fecha de vencimiento, la fecha en la que fue pagado, 
  ///un booleano que indica si fue pagado o no, y otro para indica si tiene descuento
  pub struct Pago{
  	id: u128,
    dni_socio: u128,
    costo: u128,
    fecha_pago: Option<u64>,
    fecha_vencimiento: u64,
    pagado:bool,
    tiene_descuento: bool,
  }
  #[derive(scale::Decode, scale::Encode,PartialEq,Debug)]
  #[cfg_attr(
      feature = "std",
      derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
  )]
  ///El struct Socio guarda la informacion de cada socio del club
  ///de cada socio se conoce su dni, su categoria, la actividad, 
  /// la fecha de registro,los pagos realizados y pendientes 
  /// y el total de pagos sin atraso que tuvo el socio
  
  pub struct Socio{
    dni: u128,
    categoria: TipoCategoria,  
    actividad: Option<Actividad>,
    fecha_registro:u64,
    pagos_realizados:Vec<u128>,
    pagos_pendientes:Vec<u128>,
    pagos_sin_atrasos:u128,
  }
  #[derive(scale::Decode, scale::Encode)]
  #[cfg_attr(
      feature = "std",
      derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
  )]
  ///El struct Adicional nos guarda la informacion de un solo socio 
  /// incluyendo el dni, la categoria y los montos pagados
 
  pub struct Adicional{
    dni:Option<u128>,
    cate:Option<TipoCategoria>,
    p:Vec<u128>,
  }
  impl Club {
    ///Nos crea una instancia del Club
    ///creamos dos vectores, uno de los socios y el otro para los pagos
    ///Inicializamos las categorias con sus respectivos precios
    ///Indicamos el owner
    ///La politica la inicializamos como verdadera para q solo las direcciones autorizadas realicen operaciones
  	///Inicializamos la cantidad de meses que los socios tiene que tener los pagos consecutivos sin atrasos para obtener el descuento
    ///Incializamos el descuento que se les otorga en el proximo mes
  	#[ink(constructor)]
    pub fn new()-> Self{
    	let socios:Vec<Socio> = Vec::new();
      let pagos:Vec<Pago> = Vec::new();
      let mut precio_categoria = Mapping::new();
      precio_categoria.insert(TipoCategoria::A, &5000);
      precio_categoria.insert(TipoCategoria::B, &3000);
      precio_categoria.insert(TipoCategoria::C, &2000);
      let direcciones:Vec<AccountId>=Vec::new();
      let owner = None;
      let politica = true;
      let descuento = 30;
      let cantidad_meses = 3;
      let mut club = Self{
      	socios,
        pagos,
        precio_categoria,
        direcciones,
        owner,
        politica,
        descuento,
        cantidad_meses,
      };
      club.guardar_firma();
      club
    }
    ///
    ///Guarda el caller del owner cuando se crea el contrato
    ///
    pub fn guardar_firma(&mut self){
      self.owner = Some(self.env().caller());
    }
    ///Se fija si el owner es el que quiere realizar operaciones en el contrato y devuelve un booleano
    ///Ejemplo
    ///'''
    /// if self.es_owner(){
    ///   let suma = 1 + 1;
    /// }
    ///'''
    fn es_owner(&self)->bool{
      let mut ok = false;
      let clave = self.env().caller();
      if let Some(caller) = self.owner{
        if caller == clave{
          ok = true;
        }
      }
      ok
    }
    ///Cambia la direccion del owner, que se recibe como parámetro, solo si el owner es quien lo hace
    ///Ejemplo
    ///'''
    ///  
    /// self.set_ower(account_id);
    /// 
    ///'''
    #[ink(message)]
    pub fn set_owner(&mut self, clave:AccountId){
      if self.es_owner(){
        self.owner = Some(clave);
      }
    }
    ///Cambia el estado de la politica de activada a desactivada/desactivada a activada
    ///Ejemplo
    ///'''
    /// use club::cambiar_politica
    /// self.cambiar_politica();
    ///'''
    #[ink(message)]
    pub fn cambiar_politica(&mut self ){
     if self.es_owner(){
       self.politica = !self.politica;
     }
    }
    ///Acepta o rechaza el pedido de una direccion que quiere relizar cambios en el contrato, devuelve un booleano
    ///Un pedido se acepta si la politica está activada, o si quien quiere realizar un cambio es el owner o alguna direccion que tenga permiso
    ///Ejemplo
    ///'''
    /// if aceptar_modificaciones(){
    ///   let suma = 1 + 1;
    /// }
    ///'''
   fn aceptar_modificaciones(&self)->bool{
     let mut ok = true;
     if self.politica{
       ok = self.esta_permitido();
      }
     ok
    }
    ///Se fija si el que quiere realizar modificaciones esta permitido para hacerlo, devolviendo un booleano
    ///Los que tienen permiso son el owner y las direcciones que estan guardadas en el vector
    ///Ejemplo
    ///'''
    /// if esta_permitido(){
    ///   let suma = 1 + 1;
    /// }
    ///'''
    fn esta_permitido(&self)->bool{
      let mut ok= false;
      let clave=self.env().caller();
      if self.direcciones.contains(&clave){
        ok = true;
      } else {
        ok = self.es_owner();
      }
      return ok;
    }
    ///Agrega una nueva direccion al vector solo si quien llama es el owner y la direccion no esta en el vector, devuelve un booleano que avisa si se agrego o no
    /// Si el que llama es el owner y la direccion ya esta en el vector tira un panic indicando que ya esta.
    /// Si el que llama no es el owner, tira otro panic indicando que no es el owner
    ///Ejemplo
    ///'''
    /// let ok = self.agregar_direccion(account_id);
    /// if ok{
    ///   println!("Se agrego la direccion");
    /// }
    ///'''
    #[ink(message)]
    pub fn agregar_direccion(&mut self, clave: AccountId)->bool{
      let mut ok = false;
      if self.es_owner(){
        if !self.direcciones.contains(&clave){
          self.direcciones.push(clave);
          ok=true;
        } else {
          panic!("La direccion ya esta en el vector");
        }
      }else {
        panic!("No es el owner");
      }
      return ok;
    }
    ///Elimina una direccion del vector solo si el que llama es el owner y la direccion esta en el vector, retornando un booleano
    /// Si el que llama es el owner pero la direccion no esta en el vector, tira un panic indicando que esa direccion no esta en el vector
    /// Si el que llama no es el owner, tira otro panic indicando que no es el owner
    /// Ejemplo
    /// '''
    ///  let ok=self.eliminar_direccion(account_id);
    ///  if (ok){
    ///   println!("Se elimino la direccion"); 
    ///   }
    /// '''
    #[ink(message)]
    pub fn eliminar_direccion(&mut self, clave: AccountId)->bool{
      let mut ok = false;
      if self.es_owner(){
        let mut i = 0;
        if self.direcciones.contains(&clave){
          while self.direcciones[i] != clave{
            i+=1;
          }
          self.direcciones.remove(i);
          ok = true;
        }else{panic!("No esta guardada esta clave,por lo q no se puede eliminar");}
      } else {
        panic!("No es el owner");
      }
      ok
    }
    ///Registra un nuevo socio en el club y devuelve un booleano que avisa si se registro o no
    ///La informacíon del socio se recibe por parametro
    ///Se necesita saber el dni del socio, la categoria y su actividad
    ///Para registrarlo necesita saber si se acepta el pedido de modificacion, y se asegura que el socio a registrar no exista ya
    ///crear el socio y lo pushea en el vector de socios del Club
    ///Tira un panic en caso que el socio ya exista o si la direccion que intenta registrar a un socio no está perimitida o si la categoria no es la correcta
    ///Ejemplo
    ///'''
    /// let ok = self.registrar_socio(
    /// let v = crear_vehiculo();
    ///'''
    #[ink(message)]
    pub fn registrar_socio(&mut self, dni: u128, cate: String, act: String)->bool {
      let mut ok=false;
      if self.aceptar_modificaciones(){
        if !self.existe_socio(dni){
          let categoria = match &cate as &str{
            "A"=> TipoCategoria::A,
            "B"=> TipoCategoria::B,
            "C" => TipoCategoria::C,
            _=> panic!("No se encontro la categoria"),
          };
      		let costo_mensual = self.calcular_precio(&categoria);
      		let fecha_registro = self.env().block_timestamp();
      		let id_pago = self.crear_pago_pendiente(dni, costo_mensual, fecha_registro);
      		let pagos_realizados:Vec<u128> = Vec::new();
      		let mut pagos_pendientes:Vec<u128>= Vec::new();
      		pagos_pendientes.push(id_pago);
          let pagos_sin_atrasos=0 as u128;
    			let socio:Socio=Socio::crear_socio(dni, cate, act, fecha_registro, pagos_realizados, pagos_pendientes, pagos_sin_atrasos);
      		self.socios.push(socio);
          ok=true;
        }else{panic!("ya existe el socio");}
      }else{panic!("no esta permitido para esta clave");}
      return ok;
    }
    ///Se le debe pasar la categoria para que calcule el precio de esa categoria y lo devuelve
    ///Obtiene el precio mediante el Mapping de precios del strcut Club
    ///Tira un panic en caso de que no exista dicha categoria
    ///Ejemplo
    ///'''
    /// let precio = self.calcular_precio(&TipoCategoria::A);
    /// let v = crear_vehiculo();
    ///'''
    fn calcular_precio(&self, categoria: &TipoCategoria)->u128{
    	let precio = match categoria{
      	TipoCategoria::A => self.precio_categoria.get(categoria),
        TipoCategoria::B => self.precio_categoria.get(categoria),
        TipoCategoria::C => self.precio_categoria.get(categoria),
      };
      if let Some(p) = precio{
      	return p
      }
      panic!("No hay precio para esa categoria");
    }
    ///Crea el primer pago pendiente de cada socio cuando se registran y lo pushea en el vector de pagos del club
    ///Recibe el dni del socio, el costo del pago, y la fecha de registro del socio para calcular la fecha de vencimiento
    ///
    ///Ejemplo
    ///'''
    /// self.crear_pago_pendiente(11444666, 5000, timestamp);
    /// 
    ///'''
    fn crear_pago_pendiente(&mut self, dni_socio: u128, costo: u128, fecha_registro: u64)->u128{
    	let id = self.pagos.len() as u128 + 1;
      let fecha_pago: Option<u64>;
      fecha_pago = None;
      let pagado=false;
      let tiene_descuento=false;
      let fecha_vencimiento = fecha_registro + (10 * 24 * 60 * 60); //le suma 10 dias a la fecha de registro
      let pago:Pago=Pago::crear_pago(dni_socio, id, costo, fecha_vencimiento, fecha_pago, pagado, tiene_descuento);
      self.pagos.push(pago);
      return self.pagos.len() as u128;
    }
    ///Nos busca en el vector del socios del struct club el socio
    ///Le mandamos por parametro el dni correspondiente y a traves de ese dato, busca si existe un socio con el mismo dni
    ///Si lo encuntra devuelve la poscion en donde la encontro, en caso contrario devuleve "None" indicando que no se encontro ningun socio con ese dni
    ///Ejemplo
    ///'''
    ///   let ok:bool;
    ///		let i:Option<usize>=self.buscar_socio(44851840);
    ///		let Some(p)=i{
    ///			ok=true;
  	///		}	else{ok=false;} 
    ///	'''
    fn buscar_socio(&self,dni_socio:u128)->Option<usize>{
    	let mut ok=false;
      let mut i=0;
      while i < self.socios.len() && !ok{
      	if self.socios[i].dni == dni_socio{
        	ok=true
        } else {
          i+=1
        }
    	}
      if ok{
      	return Some(i)
      }else{
        None
      }
    }
    ///Nos devulve la informacion del primer pago pendiente que tenga un determinado socio
    ///Le pasamos la poscion del socio que queremos
    ///Si el socio tiene pagos pendientes, nos devuelve una copia de la informacion del primer pago pendiente dentro del vector de "pagos_pendientes" del struct de socio
    ///si no tiene pagos pendientes, devuleve un "None" indicando que no tiene pagos pendientes el socio indicado
    /// Ejemplo
    /// '''
    ///   let ok:bool;
    ///   let pagos:Option<u128>=self.primero_pagos_pendientes(3 us usize);
    ///   if let Some(p)=pagos{
    ///     ok=true;
    ///   }else{
    ///     ok=false;
    ///   }
    ///'''
    fn primero_pagos_pendientes(&self, i: usize)->Option<u128>{
      self.socios[i].pagos_pendientes.first().copied()
    }
    /// Esta funcion nos registra un pago de un socio determinado, le tenemos que pasar el dni y el monto del pago.
    /// Si el socio existe, se fija si tiene pagos pendientes. Si tiene pagos pendientes, se fija si no fue pagada y si el socio esta pagando lo corresponde.
    /// Si cumple todo lo anterior, lo saco de la informacion de pagos pendientes del socio y cambio la informacion asociada al vector de pagos del correspodniente socio,por ultimo lo agrego al vector de los pagos realizados del socio.
    /// Si no cumple alguna de las anteriores condiciones, nos tira su correspondiente panic
    /// Por Ejemplo si no tiene pagos pendientes el socio, nos tira un panic "No tiene pagos pendientes el socio"
    /// Ejemplo
    ///'''
    /// self.registro_pago(44851840, 5000);
    /// 
    ///'''
    #[ink(message)]
    pub fn registro_pago(&mut self,dni_socio:u128,monto:u128){
      if self.aceptar_modificaciones(){
        //busco el socio 
        let i=self.buscar_socio(dni_socio);
        if let Some(i)=i{
          //me fijo si tiene pagos pendientes
          if self.socios[i].pagos_pendientes.len() > 0{
            //si tiene pagos pendientes, accedo a la primera posicion de pagos pendientes obteniendo la posicion en la q esta guardados los pagos en el vector de pagos
            let id_pendiente=self.primero_pagos_pendientes(i);
            if let Some(id)=id_pendiente{ 
              //si no fue pagada
              let pos:usize=(id - 1).try_into().unwrap();
              if self.pagos[pos].pagado == false  {
                //si el socio paga lo q le corresponde
                if self.pagos[pos].costo == monto{
                  //lo saco de la informacion de pagos pendientes del socio
                  self.socios[i].pagos_pendientes.remove(0);
                  //cambio el pago asociado al vector de pagos
                  self.pagos[pos].pagado=true; 
                  let fecha_pago = Some(self.env().block_timestamp()); //la fecha de hoy
                  self.pagos[pos].fecha_pago=fecha_pago;
                  //lo agrego a pagos realizados del socio
                  self.socios[i].pagos_realizados.push(id);
                  
                }else{
                  panic!("No paga lo q corresponde para su categoria");
                }
              }
            } 
          }else{
            panic!("No tiene pagos pendientes el socio");
          }    
        } else{
          panic!("El socio no existe");

        } 
      }else{
        panic!("No esta permitido");
      } 
    }
    ///Esta funcion nos devuelve la informacion de un socio determinado, nos indica su dni, categoria y los pagos realizados
    ///Le debemos pasar el dni, si le pasamos un dni="None" nos devulve un dni y categoria con valor "None" y un listado de los costos de los ultimos 30 pagos realizados.
    ///En caso contrario, si le pasamos un dni valido, lo busca y si encuntra el socio devuleve el dni y categoria correspondiente, con el listado de los costos de los pagos realizados.
    ///Si no encuntra el socio, tira un panic "No se encontro el socio"
    /// Ejemplo
    ///'''
    /// let a:Adicional=self.consultar_pago(Some(44851840));
    /// assert_eq!(a.dni,44851840);
    ///'''
    
    #[ink(message)]
    pub fn consultar_pagos(&self,dni_socio:Option<u128>)->Adicional{
      if self.aceptar_modificaciones(){
        let p ;
        let mut dni=None;
        let mut cate=None;
        if let Some(id)=dni_socio{
          dni = Some(id);
          let i=self.buscar_socio(id);
          if let Some(pos)=i{
            cate=Some(self.socios[pos].categoria);
            p=self.pagos.iter().filter(|pago| pago.dni_socio == id).map(|pago| pago.costo).collect();

          }else{panic!("No se encontro el socio")}
        } else {
         p= self.pagos.iter().rev().take(30).map(|pago| pago.costo).collect();
        }
       Adicional{
         dni,
         cate,
         p,
       }
      }else{panic!("No esta permitido")}
    }
    ///Recibe un monto, calcula el % de descuento de dicho monto y lo retorna, si no lo pudo hacer, devuelve un None
    ///Ejemplo
    ///'''
    /// let porciento_del_descuento = self.descuento_otorgado(5000);
    ///
    ///'''
    fn descuento_otorgado (&self,monto:u128)->Option<u128>{
      let i=monto.checked_mul(self.descuento);
      if let Some(a)=i{
        let s=a.checked_div(100);//le aplico un 30% de descuento 
        if let Some(d)=s{
          return Some(d as u128);
        }
      }
      return None;
    }
    
    ///Nos cambia el descuento que se le otorga a los socios cuando pagan una cantidad determinada de veces consecutivas sin atrasos
    ///Le pasamos el descuento nuevo que otorgamos
    /// Ejemplo
    /// '''
    ///   self.set_descuento(5000);
    ///   assert_eq!(self.descuento,5000);
    /// '''
    #[ink(message)]
    pub fn set_descuento(&mut self, num: u128){
      if self.aceptar_modificaciones(){
        self.descuento = num;
      }
    }
    
    ///Nos cambia la cantidad de meses que el socio deberia pagar sin atrasos para obtener el descuento.
    ///Debemos pasarle el numero que queremos cambiar
    ///Ejemplo
    /// '''
    ///   self.set_cantidad_meses(5);
    ///   assert_eq!(self.cantidad_meses,5);
    /// '''
    #[ink(message)]
    pub fn set_cantidad_meses(&mut self, num: u128){
      if self.aceptar_modificaciones(){
        self.cantidad_meses = num;
      }
    }
    ///Crea un nuevo pago y lo pushea en el vector de pagos del club, solo si se acepta el pedido y existe el socio, retorna si se creo o no
    ///Recibe como parametro el dni del socio
    ///Busca al socio para obtener los datos restantes
    ///Calcula la fecha de vencimiento del pago mediante la fecha de vencimiento del pago anterior
    ///Tambien se fija si el pago va a tener descuento o no, para saber eso se fija si la cantidad de meses sin pagos atrasados consecutivos es la que pide el club
    ///Tira panic si el socio o la categoria no existe
    ///Ejemplo
    ///'''
    /// let ok = self.crear_pago(11444666);
    /// if ok{
    ///   println!("Se creo el pago");
    /// }
    ///'''
    #[ink(message)]
    pub fn crear_pagos(&mut self, dni_socio: u128)->bool{
      if self.aceptar_modificaciones(){
        if self.existe_socio(dni_socio){
          let ultimos_pagos:Vec<&Pago>= self.pagos.iter().rev().filter(|pago| pago.dni_socio == dni_socio).map(|pago| pago).take(self.cantidad_meses as usize).collect();
          let b = ultimos_pagos.iter().filter(|pago| !pago.esta_vencido() && !pago.tiene_descuento).count();
          let mut ok = false;
          //busco la categoria del socio,obtengo la posicion 
          let pos=self.buscar_socio(dni_socio); //devuelve un Option 
          if let Some(p) = pos{
            let fecha = self.pagos.iter().rev().filter(|pago|  pago.dni_socio == dni_socio).map(|pago| pago.fecha_vencimiento).next();
            if let Some(f)=fecha{ 
              let monto= self.precio_categoria.get(&self.socios[p].categoria);
              if let Some(mut costo)=monto{ 
                if b as u128 == self.cantidad_meses {
                  //se otorga el descuento
                  let des=self.descuento_otorgado(costo);
                  if let Some(d)=des{
                    let i=costo.checked_sub(d);
                    if let Some(a)=i{
                       costo=a;
                       ok=true;
                    }  
                  }
                }
                let id = self.pagos.len() as u128 + 1;
                let fecha_pago: Option<u64>;
                fecha_pago = None;
                let fecha_vencimiento=f+ (30 * 24 * 60 * 60);
                
                let pago:Pago=Pago::crear_pago(dni_socio, id, costo, fecha_vencimiento, fecha_pago, false, ok);
               
                self.pagos.push(pago);
                self.socios[p].pagos_pendientes.push(self.pagos.len() as u128);
                
                
              } else{
                panic!("No hay precio para esa categoria");
              } 
            }
          }
        }else{
          panic!("No se encontro socio");
        }
        return true;
        
    	}return false;
    }  
    ///Nos cambia el monto de la categoria "a", debemos pasarle el monto por el cual lo cambiamos
    /// Ejemplo
    /// '''
    ///   self.set_categoria_a(10000);
    ///   assert_eq!(self.get_categoria_a,10000);
    /// '''
    #[ink(message)]
    pub fn set_categoria_a(&mut self, monto:u128){
      if self.aceptar_modificaciones(){
        self.precio_categoria.insert(TipoCategoria::A, &monto);
      }
    }
    ///Nos cambia el monto de la categoria "b", debemos pasarle el monto por el cual se va a cambiar.
    /// Ejemplo
    /// '''
    ///   self.set_categoria_b(5000);
    ///   assert_eq!(self.get_categoria_b,5000);
    /// '''
    #[ink(message)]
    pub fn set_categoria_b(&mut self, monto:u128){
      if self.aceptar_modificaciones(){
      	self.precio_categoria.insert(TipoCategoria::B, &monto);
      }
    }
    ///Nos cambia el monto de la categoria "c", debemos pasarle el monto por el cual se cambia.
    /// Ejemplo
    /// '''
    ///   self.set_categoria_c(3000);
    ///   assert_eq!(self.get_categoria_c,3000);
    /// '''
    #[ink(message)]
    pub fn set_categoria_c(&mut self, monto:u128){
      if self.aceptar_modificaciones(){
      	self.precio_categoria.insert(TipoCategoria::C, &monto);
      }
    }
    ///Si nos acepta el pedido la funcion "aceptar_modificaiones", devulve el monto de la categoria "a" 
    /// si no devuelve un "None" 
    /// Ejemplo
    /// '''
    ///   assert_eq!(self.get_categoria_a,5000);
    /// '''
    
    #[ink(message)]
    pub fn get_categoria_a(&mut self)->Option<u128>{
      let mut precio=None;
      if self.aceptar_modificaciones(){
      	precio=self.precio_categoria.get(TipoCategoria::A);
      }
      return precio
    }
    ///Si nos acepta el pedido la funcion "aceptar_modificaiones", devulve el monto de la categoria "b" 
    /// si no devuelve un "None" 
     /// Ejemplo
    /// '''
    ///   assert_eq!(self.get_categoria_b,3000);
    /// '''
    #[ink(message)]
    pub fn get_categoria_b(&mut self)->Option<u128>{
      let mut precio=None;
      if self.aceptar_modificaciones(){
      	precio=self.precio_categoria.get(TipoCategoria::B);
      }
      return precio
    }
    ///Si nos acepta el pedido la funcion "aceptar_modificaiones", devulve el monto de la categoria "c" 
    /// si no devuelve un "None" 
    /// Ejemplo
    /// '''
    ///   assert_eq!(self.get_categoria_c,2000);
    /// '''
    #[ink(message)]
    pub fn get_categoria_c(&mut self)->Option<u128>{
      let mut precio=None;
      if self.aceptar_modificaciones(){
      	precio=self.precio_categoria.get(TipoCategoria::C);
      }
      return precio
    }
    ///Busca y retorna un booleano si existe o no el dni del socio recibido como parametro
    ///Ejemplo
    ///'''
    /// if existe_socio(11444666){
    ///   suma = 1 + 1;
    /// }
    /// let v = crear_vehiculo();
    ///'''
    fn existe_socio(&self, dni: u128)->bool{
      let mut ok = false;
      let mut i = 0;
      while i<self.socios.len() && !ok{
        if self.socios[i].dni == dni {
          ok = true;
        }
        i+=1;
      }
      return ok
    }
    
    ///devulve un listado  de los dni de los socios, si no retorna un vector vacio 
    /// Ejemplo
    /// '''
    ///   let ok:bool;
    ///   let vec:Vec<u128>=self.get_socio();
    ///   if vec.len()==0{
    ///     ok=false;   
    ///   }else{ok=true;}
    /// '''
    #[ink(message)]
    pub fn get_socios(&self)->Vec<u128>{
      let mut vec:Vec<u128>=Vec::new();
      if self.aceptar_modificaciones(){
        for socio in &self.socios{
          vec.push(socio.dni);
        }
      }
      return vec;
    }
    
    ///Si nos acepta el pedido la funcion 'aceptar_modificaciones' y encuentra un socio devulve un listado con toda la informacion del dni, fecha de vencimiento, un booleano que indica si fue pagado por el socio y el costo de los pagos del socio que se le pasa por parametro
    /// si no lo acepta, devuelve un vector vacio 
    /// si no encunetra el socio, tira un panic informando que no existe
    /// Ejemplo
    /// '''
    ///   let ok:bool;
    ///   let vec:Vec<u128>=self.get_pago(44851840);
    ///   if vec.len()==0{
    ///     ok=false;   
    ///   }else{ok=true;}
    /// '''
    #[ink(message)]
    pub fn get_pago(&self, dni: u128)->Vec<(u128, u64, bool,u128)>{
      let mut vec: Vec<(u128,u64,bool,u128)>=Vec::new();
      if self.aceptar_modificaciones(){
        if self.existe_socio(dni){
          for precio in &self.pagos{
            if precio.dni_socio == dni{
              let tupla = (precio.id, precio.fecha_vencimiento, precio.pagado, precio.costo);
              vec.push(tupla);
              
            }
          }
        }else{
          panic!("No existe el socio");
        }
      }
      return vec;
    }

    ///Recibe la posicion de un socio, devolviendo la categoria y la actividad del socio en caso de tener alguna
    ///Retorna la informacion en formato String en caso de que se acepten las modificaciones
    /// Si la funcion aceptar_modificaciones devuelve false, tira un panic informando que no se acepto el pedido
    /// Si no se encontro la categoria, tira un panic 
    ///Ejemplo
    ///'''
    /// let info_socio = self.get_info_socio(0);
    ///'''
    #[ink(message)]
    pub fn get_info_socio(&self,i:u128)->(String, String){
      if self.aceptar_modificaciones(){
        let cate:String;
        if i <  self.socios.len() as u128{
          cate = match self.socios[i as usize].categoria{
          	TipoCategoria::A=> "A".to_string(),
          	TipoCategoria::B=> "B".to_string(),
          	TipoCategoria::C=> "C".to_string(),
          	_=>panic!("No existe esta categoria"),
        	};
        	let mut act = "NADA".to_string();
          if let Some(a) = &self.socios[i as usize].actividad{
          	act = match a{
              Actividad::FUTBOL=>"FUTBOL".to_string(), 

            	Actividad::BASQUET=>"BASQUET".to_string(), 

            	Actividad::RUGBY=>"RUGBY".to_string(), 

            	Actividad::HOCKEY=>"HOCKEY".to_string(), 

            	Actividad::NATACION=>"NATACION".to_string(), 

            	Actividad::TENIS=>"TENIS".to_string(),

            	Actividad::PADDLE=>"PADDLE".to_string(),
              
              Actividad::TODOS=>"TODOS".to_string(),

            	_=>panic!("Actividad invalida"),
            };
          
          }
          return (cate, act);
        }else{panic!("No es la posicion de socio");}
      }
      panic!("No se acepta el pedido de lectura");
    }
  }
    
  impl Socio{

    ///Crea un nuevo socio y lo retorna
    ///Al dni del socio lo obtiene por parametro, y tambien recibe señaladores para elegir la categoria y la actividad
    ///Obtiene el costo mensual del socio, la fecha de registro, y llama a otra funcion que cree un nuevo pago pendiente
    ///Si la categoria enviada no coinciden con las que tiene el club, tira un panic informandolo
    ///Ejemplo
    ///'''
    /// let socio = self.socio(11444666, "A".to_string(), "FUTBOL".to_string());
    /// assert_eq!(socio.dni,11444666);
    ///'''
    pub fn crear_socio(dni: u128, cate: String, act:String, fecha_registro: u64, pagos_realizados: Vec<u128>, pagos_pendientes: Vec<u128>,pagos_sin_atrasos:u128)-> Socio{
    	let categoria = match &cate as &str{
      	"A"=> TipoCategoria::A,
        "B"=> TipoCategoria::B,
        "C" => TipoCategoria::C,
        _=> panic!("No se encontro la categoria"),
      };
      let actividad = match categoria{
        TipoCategoria::A=> Some(Actividad::TODOS),
        TipoCategoria::B=>Socio::elegir_actividad(act),
        TipoCategoria::C=> None,
      };
      Socio{
      	dni,
        categoria,
        actividad,
        fecha_registro,
        pagos_realizados,
        pagos_pendientes,
        pagos_sin_atrasos,
      }
    }
		///Matchea y devuelve la categoria del socio
    ///Tira un panic en caso de que se una actividad invalida
    ///Ejemplo
    ///'''
    /// let actividad = self.elegir_actividad("HOCKEY".to_string());
    /// assert_eq!(Some(actividad),Actividad::HOCKEY);
    ///'''
    fn elegir_actividad(act: String)->Option<Actividad>{
    	let actividad = match &act as &str{
      	"FUTBOL"=> Some(Actividad::FUTBOL),
        "BASQUET"=> Some(Actividad::BASQUET), 
        "RUGBY"=> Some(Actividad::RUGBY), 
        "HOCKEY"=> Some(Actividad::HOCKEY), 
        "NATACION"=>Some(Actividad::NATACION), 
        "TENIS"=> Some(Actividad::TENIS),
        "PADDLE"=> Some(Actividad::PADDLE),
        _=> panic!("Actividad invalida"),
      };
      actividad
    }
   
  }
  impl Pago{
    
    ///Crea un pago del socio pasado por parametro y lo devuelve
    ///Recibe el dni del socio, el costo del pago, la fecha de vencimiento, fecha de pago, si fue pagado o no y si tiene descuento o no
    ///Ejemplo
    ///'''
    /// self.crear_pago(11444666, 5000, timestamp,None,true,true);
    /// 
    ///'''
    pub fn crear_pago( dni_socio: u128,id:u128, costo: u128, fecha_vencimiento:u64,fecha_pago:Option<u64>,ok:bool,descuento:bool)->Pago{
     
      let pagado=ok;
      let tiene_descuento=descuento;
      let pago=Pago{
      	id,
        dni_socio,
        costo,
        fecha_vencimiento,
        fecha_pago,
        pagado,
        tiene_descuento,
      };
      return pago
    }
    ///Nos indica si el socio pago atrasado o no
    /// Si la fecha de pago indicada es mayor a la fecha de vencimiento quiere decir que el pago es atrasado, esta vencido
    /// Si no, quiere decir que pago en fecha.
    ///Ejemplo
    ///'''
    /// let ok=self.esta_vencido;
    /// assert!(ok);
    /// 
    ///'''
    fn esta_vencido(&self)->bool{
      let mut ok = false;
      if let Some(fecha) = self.fecha_pago{
        if fecha > self.fecha_vencimiento{
          ok = true;
        }
      } 
      ok
    }
    }
  
  //TEST
  
  #[cfg(test)]
  mod test{
    use ink::primitives::AccountId;

    use super::*;
    
    #[ink::test]
    #[should_panic(expected = "Esta vencida")]
    fn esta_vencido_test(){
      //registro  socio
      let mut c=Club::new();
      let clave1=[1; 32].into();
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      c.crear_pagos(543);
      c.registro_pago(543,2000);
      let p=c.pagos.last();
      if let Some(g)=p{
        let vencida=g.esta_vencido();
        assert!(vencida,"Esta vencida");
      }
    }
    #[ink::test]
    fn crear_pago_test(){
      //registro  socio
      let mut c=Club::new();
      let clave1=[1; 32].into();
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      c.crear_pagos(543);
      let mut pago:Pago;
      let fecha_registro=000002345233;
      let id = c.pagos.len() as u128 + 1;
      let fecha_pago: Option<u64>;
      fecha_pago = None;
      let pagado=false;
      let tiene_descuento=false;
      let fecha_vencimiento = fecha_registro + (10 * 24 * 60 * 60);
      let pago:Pago=Pago::crear_pago(543, id, 2000, fecha_vencimiento, fecha_pago, pagado, tiene_descuento);
      assert_eq!(pago.dni_socio,543);
    }
    #[ink::test]
    #[should_panic(expected = "No se encontro socio")]
    fn crear_pagos_socio_invalidos_test(){
      //registro  socio
      let mut c=Club::new();
      let clave1=[1; 32].into();
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      c.crear_pagos(44851840);
    }
    #[ink::test]
    fn crear_pagos_test(){
      //registro  socio
      let mut c=Club::new();
      let clave1=[1; 32].into();
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      c.crear_pagos(543);
      c.crear_pagos(543);
      c.crear_pagos(543);
    }
    #[ink::test]
    fn crear_socio_test(){
      let pagos_realizados:Vec<u128>=Vec::new();
      let pagos_pendientes:Vec<u128>=Vec::new();
      let socio :Socio= Socio::crear_socio(44581840, "A".to_string(), "TODOS".to_string(), 0045675675, pagos_realizados, pagos_pendientes, 10);
      assert_eq!(socio.dni, 44581840);
    }
    #[ink::test]

    #[should_panic(expected = "No se encontro la categoria")]
    fn crear_socio_invalido_test(){
      let pagos_realizados:Vec<u128>=Vec::new();
      let pagos_pendientes:Vec<u128>=Vec::new();
      let socio :Socio= Socio::crear_socio(44581840, "D".to_string(), "TODOS".to_string(), 0045675675, pagos_realizados, pagos_pendientes, 10);
      assert_eq!(socio.dni, 44581840);
    }
    
    #[ink::test]
    fn elegir_actividad_futbol_test(){
      let pagos_realizados:Vec<u128>=Vec::new();
      let pagos_pendientes:Vec<u128>=Vec::new();
      let socio :Socio= Socio::crear_socio(44581840, "A".to_string(), "TODOS".to_string(), 0045675675, pagos_realizados, pagos_pendientes, 10);
      
    	let act= Socio::elegir_actividad("FUTBOL".to_string());
      if let Some(a) = act{
        assert_eq!(a, Actividad::FUTBOL);
      }
    }
    #[ink::test]
    fn elegir_actividad_basquet_test(){
      let pagos_realizados:Vec<u128>=Vec::new();
      let pagos_pendientes:Vec<u128>=Vec::new();
      let socio :Socio= Socio::crear_socio(44581840, "A".to_string(), "TODOS".to_string(), 0045675675, pagos_realizados, pagos_pendientes, 10);
    	let act = Socio::elegir_actividad("BASQUET".to_string());
      if let Some(a) = act{
        assert_eq!(a, Actividad::BASQUET);
      }
    }
    
    #[ink::test]
    fn elegir_actividad_rugby_test(){
      let act = Socio::elegir_actividad("RUGBY".to_string());
      if let Some(a) = act{
        assert_eq!(a, Actividad::RUGBY);
      }
    }
    #[ink::test]
    fn elegir_actividad_hockey_test(){
      let act = Socio::elegir_actividad("HOCKEY".to_string());
      if let Some(a) = act{
        assert_eq!(a, Actividad::HOCKEY);
      }
    }
    #[ink::test]
    fn elegir_actividad_natacion_test(){

    	let act = Socio::elegir_actividad("NATACION".to_string());
      if let Some(a) = act{
        assert_eq!(a, Actividad::NATACION);
      }
    }
    #[ink::test]
    fn elegir_actividad_tenis_test(){
      let act = Socio::elegir_actividad("TENIS".to_string());
      if let Some(a) = act{
        assert_eq!(a, Actividad::TENIS);
      }
    }
    #[ink::test]
    fn elegir_actividad_paddle_test(){
      let act = Socio::elegir_actividad("PADDLE".to_string());
      if let Some(a) = act{
        assert_eq!(a, Actividad::PADDLE);
      }
    }
    
    #[ink::test]
    #[should_panic(expected = "Actividad invalida")]
    fn elegir_actividad_todos_test(){
      let act = Socio::elegir_actividad("TODOS".to_string());
      if let Some(a) = act{
        assert_eq!(a, Actividad::TODOS);
      }
    }
    
    #[ink::test]
    fn get_info_socio_test(){ 
      let mut club = Club::new();
      club.registrar_socio(44581840, "B".to_string(), "NATACION".to_string());
      club.registrar_socio(44851840, "A".to_string(), "TODOS".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "A","Error con categoria");
      assert_eq!(b,"TODOS","Error con actividad");
    }
    #[ink::test]
    fn get_info_socio_2_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "A".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "B".to_string(), "NATACION".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "B","Error con categoria");
      assert_eq!(b,"NATACION","Error con actividad");
    }
    #[ink::test]
    fn get_info_socio_3_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "C".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "B".to_string(), "FUTBOL".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "B","Error con categoria");
      assert_eq!(b,"FUTBOL","Error con actividad");
    }
    #[ink::test]
    fn get_info_socio_4_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "C".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "C".to_string(), "FUTBOL".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "C","Error con categoria");
      assert_eq!(b,"NADA","Error con actividad");
    }
    #[ink::test]

    #[should_panic(expected = "No se encontro la categoria")]
    fn get_info_socio_5_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "C".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "D".to_string(), "FUTBOL".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "D","Error con categoria");
      assert_eq!(b,"NADA","Error con actividad");
    }
    #[ink::test]
    fn get_info_socio_6_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "C".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "B".to_string(), "BASQUET".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "B","Error con categoria");
      assert_eq!(b,"BASQUET","Error con actividad");
    }
    #[ink::test]
    fn get_info_socio_7_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "C".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "B".to_string(), "RUGBY".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "B","Error con categoria");
      assert_eq!(b,"RUGBY","Error con actividad");
    }
    #[ink::test]
    fn get_info_socio_8_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "C".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "B".to_string(), "HOCKEY".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "B","Error con categoria");
      assert_eq!(b,"HOCKEY","Error con actividad");
    }
    #[ink::test]
    fn get_info_socio_9_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "C".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "B".to_string(), "TENIS".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "B","Error con categoria");
      assert_eq!(b,"TENIS","Error con actividad");
    }
    #[ink::test]
    fn get_info_socio_10_test(){
      let mut club = Club::new();
      club.registrar_socio(44581840, "C".to_string(), "TODOS".to_string());
      club.registrar_socio(44851840, "B".to_string(), "PADDLE".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "B","Error con categoria");
      assert_eq!(b,"PADDLE","Error con actividad");
    }
    #[ink::test]
    #[should_panic(expected = "Actividad invalida")]
    fn get_info_socio_nada_test(){ 
      let mut club = Club::new();
      club.registrar_socio(44581840, "B".to_string(), "NATACION".to_string());
      club.registrar_socio(44851840, "B".to_string(), "CORRER".to_string());
      let t=club.get_info_socio(1);
      let a=t.0;
      let b=t.1;
      assert_eq!(a, "B","Error con categoria");
     
    }
   
    #[ink::test] 
    fn get_socios_test(){
      //creamos una instancia del contrato
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      
      let v=c.get_socios();
      assert_eq!(v.len(),c.socios.len(),"Esta mal");
    }
    
    #[ink::test]
    fn get_pago_test(){
       //creamos una instancia del contrato
       let mut c=Club::new();
       //mockeamos una clave
       let clave1=[1; 32].into();
       //la agregamos al vector
       c.agregar_direccion(clave1);
       c.registrar_socio(234, "A".to_string(), "todos".to_string());
       let clave2=[2;32].into();
       c.agregar_direccion(clave2);
       c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
       let clave3=[3;32].into();
       c.agregar_direccion(clave3);
       c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
       c.registro_pago(178, 3000);
       c.registro_pago(543, 2000);
       c.registro_pago(234, 5000);
      let t=c.get_pago(178);
      assert_eq!(t.len(),1);
    }
    #[ink::test]
    #[should_panic(expected = "No existe el socio")]
    fn get_pago_invalido_test(){
       //creamos una instancia del contrato
       let mut c=Club::new();
       //mockeamos una clave
       let clave1=[1; 32].into();
       //la agregamos al vector
       c.agregar_direccion(clave1);
       c.registrar_socio(234, "A".to_string(), "todos".to_string());
       let clave2=[2;32].into();
       c.agregar_direccion(clave2);
       c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
       let clave3=[3;32].into();
       c.agregar_direccion(clave3);
       c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
       c.registro_pago(178, 3000);
       c.registro_pago(543, 2000);
       c.registro_pago(234, 5000);
      let t=c.get_pago(44851840);
      assert_eq!(t.len(),1);
    }
    
    #[ink::test]
    fn no_existe_socio_test(){
      let  club = Club::new();
      let ok = club.existe_socio(44581840);
      assert!(!ok);
    }
    #[ink::test]
    fn existe_socio_test(){
      let mut club = Club::new();    
      let clave: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      club.agregar_direccion(clave);
      club.registrar_socio(44581840, "A".to_string(), "FUTBOL".to_string());
      let ok = club.existe_socio(44581840);
      assert!(ok);
    }
    #[ink::test]
    fn set_categoria_a_test(){
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.set_categoria_a(7000);
      assert_eq!(c.get_categoria_a(),Some(7000));
    }
    #[ink::test]
    fn set_categoria_b_test(){
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.set_categoria_b(5000);
      assert_eq!(c.get_categoria_b(),Some(5000));

    }
    #[ink::test]
    fn set_categoria_c_test(){
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.set_categoria_c(3000);
      assert_eq!(c.get_categoria_c(),Some(3000));
    }
    
    #[ink::test]
    fn set_cantidad_meses_test(){
      let mut club=Club::new();
      let cant=club.cantidad_meses;
      club.set_cantidad_meses(20);
      assert_ne!(club.cantidad_meses, cant);
    }
    
    #[ink::test]
    fn set_descuento_test(){
      let mut club=Club::new();
      let cant=club.descuento;
      club.set_descuento(20);
      assert_ne!(club.descuento, cant);
    }
    
    #[ink::test]
    fn descuento_otorgado_test(){
      let  club = Club::new();
      let monto = club.descuento_otorgado(100);
      if let Some(l)=monto{
        assert_eq!(l, 30,"Dio error el descuento");
      }
        
    }
  
    #[ink::test]
    fn agregar_direccion_nueva_test(){
      let mut club = Club::new();     
      let account_id: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      let resultado = club.agregar_direccion(account_id);
      assert_eq!(resultado,true);
    
    }
    #[ink::test]
    #[should_panic(expected = "La direccion ya esta en el vector")]
    fn agregar_direccion_repetida_test(){
      let mut club = Club::new();   
      let account_id: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      club.agregar_direccion(account_id);
      assert!(club.agregar_direccion(account_id),"La clave ya esta guardada");
    }
    #[ink::test]
    fn eliminar_direccion_test(){
      let mut club = Club::new();     
      let account_id1: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      club.agregar_direccion(account_id1);
      let account_id2:AccountId=[0;32].into();
      club.agregar_direccion(account_id2);
      club.eliminar_direccion(account_id1);
    }
    #[ink::test]
    fn eliminar_direccion_dos_test(){
      let mut club = Club::new();     
      let account_id1: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      club.agregar_direccion(account_id1);
      let account_id2:AccountId=[0;32].into();
      club.agregar_direccion(account_id2);
      let account_id3:AccountId=[1;32].into();
      club.agregar_direccion(account_id3);
      let account_id4:AccountId=[2;32].into();
      club.agregar_direccion(account_id4);
      club.eliminar_direccion(account_id3);
    }
    #[ink::test]
    #[should_panic(expected = "No esta guardada esta clave,por lo q no se puede eliminar")]
    fn eliminar_direccion_invalida_test(){
      let mut club = Club::new();     
      let account_id1: AccountId = [0x42; 32].into(); // Crear un AccountId mock
   
      club.eliminar_direccion(account_id1);
    }
    #[ink::test]
    fn esta_permitido_test(){
      let mut club = Club::new();   
      let account_id: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      club.agregar_direccion(account_id);
      let okey = club.esta_permitido();
      assert!(okey,"No esta permitida la clave");
    }
    
    #[ink::test]
    fn es_owner_test(){
      let mut club=Club::new();
      assert!(club.es_owner());
      let clave: AccountId = [0x42; 32].into();
      club.set_owner(clave);
      assert_eq!(club.es_owner(), false);
    }
    #[ink::test]
    fn cambiar_politica_test(){
      let mut club=Club::new();
      assert!(club.politica);
      club.cambiar_politica();
      assert_eq!(club.politica, false);
    }
    #[ink::test]
    fn aceptar_modificaciones_test(){
      let mut club=Club::new();
      assert!(club.aceptar_modificaciones());
      let clave: AccountId = [0x42; 32].into();
      club.set_owner(clave);
      assert_eq!(club.es_owner(), false);
    }
    
    #[ink::test]
    fn set_owner_test(){
      let mut club=Club::new();
      let clave: AccountId = [0x42; 32].into();
      club.set_owner(clave);
      if let Some(owner) = club.owner{
        assert_eq!(owner, clave);
      }
    }
    #[ink::test]
    fn guardar_firma_test(){
      let mut club = Club::new();
      club.guardar_firma();
      assert_ne!(club.owner, None);
    }
    #[ink::test]
    fn crear_pago_pendiente_test(){
      let mut club=Club::new();
      club.crear_pago_pendiente(44666785, 5000, 334545654);
      assert_eq!(club.pagos.len(), 1);
    }
    
    #[ink::test]
    fn calcular_precio_a_test(){
      let c=Club::new();
      let precio = c.calcular_precio(&TipoCategoria::A);
      assert_eq!(precio, 5000);
    }
    #[ink::test]
    fn calcular_precio_b_test(){
      let c=Club::new();
      let precio = c.calcular_precio(&TipoCategoria::B);
      assert_eq!(precio, 3000);
    }
    #[ink::test]
    fn calcular_precio_c_test(){
      let c=Club::new();
      let precio = c.calcular_precio(&TipoCategoria::C);
      assert_eq!(precio, 2000);
    }
    
    #[ink::test]
    fn buscar_socio_test(){
      let mut club = Club::new();    
     let clave: AccountId = [0x42; 32].into(); // Crear un AccountId mock
     club.agregar_direccion(clave);
     club.registrar_socio(44581840, "B".to_string(), "NATACION".to_string());
     let o=club.buscar_socio(44581840);
     let mut ok=false;
     if let Some(s)=o{
       ok=true;
      };
     assert_eq!(ok,true,"no se encontro el socio");
    }
    #[ink::test]
    fn buscar_socio_invalido_test(){
      let mut club = Club::new();    
     let clave: AccountId = [0x42; 32].into(); // Crear un AccountId mock
     club.agregar_direccion(clave);
     club.registrar_socio(44581840, "B".to_string(), "NATACION".to_string());
     let o=club.buscar_socio(44851240);
     let mut ok=false;
     if let Some(s)=o{
       ok=true;
      };
     assert_eq!(ok,false,"no se encontro el socio");
    }
    #[ink::test]
    fn primero_pagos_pendientes_test(){
      let mut club=Club::new();
      club.registrar_socio(44851840,"B".to_string(),"NATACION".to_string());
      club.registrar_socio(22884342,"C".to_string(),"TENIS".to_string());
      let a=club.primero_pagos_pendientes(0 as usize);
      let mut ok=false;
      if let Some(c)=a{
        ok=true;
      }
      assert!(ok);
    }
    #[ink::test]
    fn registrar_socio_nuevo_test(){
      let mut club = Club::new();    
      let clave: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      club.agregar_direccion(clave);
      club.registrar_socio(44581840, "B".to_string(), "NATACION".to_string());
      let num = club.socios.len();
      assert_eq!(num, 1);
    }
    #[ink::test]
    #[should_panic(expected = "La direccion ya esta en el vector")]
    fn registrar_socio_invalido_test(){
      let mut club = Club::new(); 
      let clave: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      club.agregar_direccion(clave);
      club.registrar_socio(44581840, "B".to_string(), "NATACION".to_string());
      
      let clave2: AccountId = [0x42 ; 32].into(); // Crear un AccountId mock
      club.agregar_direccion(clave2);
      club.registrar_socio(11222333, "B".to_string(), "BASQUET".to_string());
      
      let clave3: AccountId = [0x42; 32].into(); // Crear un AccountId mock
      club.registrar_socio(44581840, "B".to_string(), "NATACION".to_string());
      
      let num = club.socios.len();
      assert_eq!(num, 2);
    }
    
    
    #[ink::test]
    fn registro_pago_test() { 
      //creamos una instancia del contrato
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      c.registro_pago(178, 3000);
      c.registro_pago(543, 2000);
      c.registro_pago(234, 5000);
     
  
    }

    #[ink::test]
    #[should_panic(expected = "No paga lo q corresponde para su categoria")]
    fn registro_pago_monto_invalido_test(){
      //creamos una instancia del contrato
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      
      c.registro_pago(178, 3000);
      
      c.registro_pago(543, 3000);
    }
    #[ink::test]
    #[should_panic(expected = "El socio no existe")]
    fn registro_pago_socio_invalido(){
      //creamos una instancia del contrato
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      c.registro_pago(44851840, 3000);
    }
    #[ink::test]
    #[should_panic(expected = "No tiene pagos pendientes el socio")]
    fn registro_pago_sin_pendientes(){
      //creamos una instancia del contrato
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      c.registro_pago(543, 2000);
      c.registro_pago(543,2000);
      c.registro_pago(543,2000);
    }

    
    #[ink::test]
    fn consultar_pagos_test(){
      //creamos una instancia del contrato
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let clave2=[2;32].into();
      c.agregar_direccion(clave2);
      c.registrar_socio(178, "B".to_string(), "FUTBOL".to_string());
      let clave3=[3;32].into();
      c.agregar_direccion(clave3);
      c.registrar_socio(543, "C".to_string(), "NINGUNA".to_string());
      let dni=Some(543);
      let ad=c.consultar_pagos(dni);
      assert_eq!(ad.dni,dni);
      assert_eq!(ad.cate,Some(TipoCategoria::C));
    }
    
    #[ink::test]
    fn consultar_pago_sin_dni(){
      //creamos una instancia del contrato
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let dni=None;
      let ad=c.consultar_pagos(dni);
      assert_eq!(ad.dni,dni);
      assert_eq!(ad.cate,None);
    }
    #[ink::test]
    #[should_panic(expected = "No se encontro el socio")]
    fn consultar_pago_invalido(){
      
      //creamos una instancia del contrato
      let mut c=Club::new();
      //mockeamos una clave
      let clave1=[1; 32].into();
      //la agregamos al vector
      c.agregar_direccion(clave1);
      c.registrar_socio(234, "A".to_string(), "todos".to_string());
      let dni=Some(44851840);
      let ad=c.consultar_pagos(dni);
      assert_eq!(ad.dni,dni);
      assert_eq!(ad.cate,Some(TipoCategoria::C));
    }
  }
} 

