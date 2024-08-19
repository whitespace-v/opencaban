import { Elysia,t  } from 'elysia'
import {Router} from "./Router";
import {Logger} from './Logger'
import {Prisma} from './Prisma'

class API {
    constructor() {
        this.useMiddlewares()
        this.useRoutes()
        new Prisma()
        this.init().then(async () => {
            Logger.out("magenta",`[server]: Server is running at http://localhost:${process.env.BACKEND_PORT}`);
        })
    }
    private app = new Elysia()

    private useMiddlewares(): API{
        return this
    }
    private useRoutes(){
        this.app.get('/', () => 'Hello Elysia')
        this.app.group('/cars', app => app.use(Router.cars))
        // this.app.group('/users', app => app.use(Router.users))
    }
    async init(){
        this.app.listen(process.env.BACKEND_PORT || 5500)

    }
}

new API()

