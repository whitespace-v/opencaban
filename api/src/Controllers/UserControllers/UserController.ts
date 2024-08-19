import {Prisma} from '../../Prisma'
export class UserController {
    static async getAllUsers(){
        // const d =  await Prisma.client.u.findMany();
        // console.log(d)
        return 'all users';
    }
    static async getUserById(id: string){
        try {
            console.log(id, 'asasdas')
            return id;
        } catch (e){
            console.log(e)
        }

    }
}
