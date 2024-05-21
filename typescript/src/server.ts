import express from "express";
import * as user from "./user";

export function startServer(ownPort: number, userBaseUrl: string) {
    const app = express();
    app.use(express.json())
    app.use("/", user.getUserRoutes(userBaseUrl));
    
    return app.listen(ownPort, () => {
        console.log(`Server is running at http://localhost:${ownPort}`);
    });
}