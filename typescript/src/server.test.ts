import { startServer } from "./server";
import { UserRequest, UserResponse } from "./user";
import { http, HttpResponse } from 'msw'
import { SetupServer, setupServer } from 'msw/node'
import axios from "axios"
import { Server } from "http";

let mockServer: SetupServer;
let realServer: Server;
let lastRequest: any;
function setupMockServer(port: number, returnValue: UserResponse) {
    const worker = setupServer(
        http.post(`http://localhost:${port}/users`, async ({request}) => {
            lastRequest = await request.json();
            return HttpResponse.json(returnValue);
        }),
    )
    worker.listen({onUnhandledRequest: 'bypass'});
}

describe("server", () => {

    afterEach(() => {
        mockServer?.resetHandlers();
        mockServer?.close();
        realServer?.close();
    })


    test("can post user", async () => {
        const userRequest: UserRequest = { name: "userName", job: "the job" };
        const userResponse: UserResponse = { name: "userName", job: "the job", id: "123" };
        setupMockServer(12345, userResponse);
        realServer = startServer(5000, "http://localhost:12345");
        const response = await axios.post<UserResponse>("http://localhost:5000/users", userRequest);
        expect(lastRequest).toEqual(userRequest);
        expect(response.status).toEqual(200);
        expect(response.data).toEqual(userResponse);
    });

    test("cannot post bad user", async () => {
        const userResponse: UserResponse = { name: "userName", job: "the job", id: "123" };
        setupMockServer(12345, userResponse);
        realServer = startServer(5000, "http://localhost:12345");
        try{
            await axios.post<UserResponse>("http://localhost:5000/users", {badRequest: "badRequest"});
            fail("should have failed with a 400")
        } catch(e: any) {
            expect(e.toString()).toEqual("AxiosError: Request failed with status code 400")
        }
    });

})