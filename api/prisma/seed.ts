import {EColor, PrismaClient} from '@prisma/client';

const client = new PrismaClient();

const carToCreate = 
    {
        body_code: '1322112312313323333s1023230-dFC6EFA',
        vin_code: '1322F1233123123333C3s262EFA-d230',
        data: {
            create: {
                body: 'TRUCK',
                color: EColor['yellow'],
                brand: 'trimo',
                year: '1992',
                model: 'HIMO ZX 320 DURA',
            }
        },
        encar: {
            create: {
                alias: 'Encar data'
            }  
        },
        ninja: {
            create: {
                bid_info: '2222',    
                price: '123'         ,
                score : '123'         ,
                place  : '123'        ,
                name     : '123'      ,
                year      : '123'     ,
                special_marks : '' ,
                images      : ['123' , '123123', '1231231']  ,
                auclist      : ['123']  ,
                car_kind_type: '123'  , 
                kaijo_code   : '123'  ,
                auction_count : '123' ,
                bid_no       : '123'  ,
                zaiko_no     : '123'  ,
                list_number   : '123' ,
                brand     : '123'    ,
                equipment: {
                    create: [{ text: "Power steering",status: true}, { text: "Power steering",status: true},{ text: "Power steering",status: true}  ]
                }
            }    
        
    }
}


const seed = async () =>{
    // await client.$connect()
    const res = await client.car.create({
        data: carToCreate
    });
    console.log(res);
    
} 
  
// count the number of users

await seed()
  .then(async () => {
    await client.$disconnect();
  })
  .catch(async (e) => {
    console.error(e);
    await client.$disconnect();
  });
