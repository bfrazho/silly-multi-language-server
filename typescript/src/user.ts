import axios from "axios";
import { IsDefined, validate } from "class-validator";
import {Request, Response, Router } from "express";

export class UserResponse {
    name: string;
    job: string;
    id: string;

    constructor(name: string, job: string, id: string){
        this.name=name;
        this.job=job;
        this.id=id;
    }
}

export class UserRequest {
    @IsDefined()
    name?: string;
    @IsDefined()
    job?: string;
    constructor(userRequest: UserRequest){
        this.name=userRequest.name;
        this.job=userRequest.job;
    }
}

class UserRouter{
    userBaseUrl: string;
    createUser = async (request: Request, response: Response) => {
        const userRequest: UserRequest = new UserRequest(request.body);
        const validationResults = await validate(userRequest);
        if(validationResults.length>0){
            return response.status(400).send(validationResults);
        }

        const axiosResponse = await axios.post<UserResponse>(`${this.userBaseUrl}/users`, userRequest);
        return response.send(axiosResponse.data);
    }

    constructor(userBaseUrl: string){
        this.userBaseUrl = userBaseUrl
    }
}

export function getUserRoutes(userBaseUrl: string): Router{
    const userRouter = new UserRouter(userBaseUrl)
    const router = Router();
    router.post("/users", userRouter.createUser)
    return router
}