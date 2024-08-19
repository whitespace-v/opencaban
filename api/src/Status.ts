export class Code{
    static NOT_FOUND(message: string){
        return {status_code: 404, text: "Not Found", message} 
    } 
    static NOT_ACCEPTABLE(message: string){
        return {status_code: 406, text: "Not Acceptable", message} 
    } 
    static CREATED(message: string){
        return {status_code: 201, text: "Created", message} 
    }
}