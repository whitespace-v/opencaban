import {EColor} from '@prisma/client';
export interface ICar 
{
    data: {
        body_code: string,
        vin_code: string,
        body: string, // then -> enum
        color: EColor,
        brand: string, // then -> enum
        year: string,
        model: string 
    },
    encar: {},
    ninja: {
            bid_info: string,    
            price: string,
            score: string,
            place: string,
            name: string,
            year: string,
            special_marks : string,
            details: {
                    type: string,
                    mileage: string,
                    color: EColor,
                    body: string,
                    ac: string,
                    displacement: string,
                    transmission: string,
                    steering: string,
                    jsi: string
            },
            images: string[]  ,
            auclist: string[]  ,
            car_kind_type: string  , 
            kaijo_code: string  ,
            auction_count: string ,
            bid_no: string  ,
            zaiko_no: string  ,
            list_number: string ,
            brand: string    ,
            equipment: {text: string, status: true}[]
        }
    }