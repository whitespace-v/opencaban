import {PrismaClient} from '@prisma/client';


export class Prisma {
    static client: PrismaClient 
    
    static do() {
        this.client.$connect();
        
        this.client.$disconnect();
    }
    constructor ( ){
        Prisma.client = new PrismaClient();
    }
}