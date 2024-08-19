import { Logger } from '../../Logger';
import {Prisma} from '../../Prisma'
import { Code } from '../../Status';
import { ICar } from '../../models';
import {EColor} from '@prisma/client';
import {Car} from '@prisma/client';

export class CarControllerSet {
    static async upsert(car: Icar){
        try{
            if (car){
                const res = await Prisma.client.car.create({
                    data: {
                        body_code: car.body_code,
                        vin_code: car.vin_code,
                        data: {create: car.data},
                        encar: {create: car.encar},
                        ninja: {
                            create: {
                                uuid: car.ninja.uuid,
                                data: { create: car.ninja.data},
                                details: { create: car.ninja.details},
                                equipment: { create: car.ninja.equipment}
                            }
                        }
                    }
                });
                return Code.CREATED(`Машина[${res.id}] VIN:${res.vin_code} BODY:${res.body_code} успешно добавлена!`) 
            } else{
                return Code.NOT_ACCEPTABLE('Некорректные данные!')
            }
            // ? car : Code.NOT_FOUND('Произошла ошибка с созданием автомобиля')  
        } catch (e) {
            Logger.out('red', e)
        }
    }
}
