import {Prisma} from '../../Prisma'
import { Logger } from '../../Logger';
import { Code } from '../../Status';
export class CarControllerGet {

    static async all(){
        try{
            const cars =  await Prisma.client.car.findMany({
                include: {
                    data: true,
                    ninja: {
                        include: {
                            data: true,
                            details: true,
                            equipment: true
                        }
                    },
                    encar: true,
              },
            });
            return cars;
        } catch (e) {
            Logger.out('red', e)
        }
    }

    static async byId(id: number){
        try{
            if (id) {
                const car =  await Prisma.client.car.findUnique({
                    where: {id},
                    include: {
                        data: true,
                        ninja: {
                            include: {
                                data: true,
                                details: true,
                                equipment: true
                            }
                        },
                        encar: true,
                  },
                });
                return car ? car : Code.NOT_FOUND('Извините, автомобилей с таким ID, к сожалению нет')
            } else {
                return Code.NOT_FOUND('Нокорректный ID')
            }

        } catch (e) {
            Logger.out('red', e)
        }
    }
}
