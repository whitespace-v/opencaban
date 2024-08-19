import {Elysia} from "elysia";
import {CarControllerGet, CarControllerSet} from "./Controllers";
import { ICar } from "./models";

export class Router {
    static cars = new Elysia()
        .get('/', () => CarControllerGet.all())
        .get('/:id', ({params: {id}}) => CarControllerGet.byId(Number(id)))
        .post('/', ({body: car}) => CarControllerSet.upsert(car))


    // static users = new Elysia()
    //     .get('/', () => CarControllerGet.getAllUsers())
    //     .get('/:id', () => UserController.getUserById())
}
