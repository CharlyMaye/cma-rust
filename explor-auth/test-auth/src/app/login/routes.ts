import { Routes } from "@angular/router";

export function getRoutes(): Routes {
    return [
        {
            path: "",
            loadComponent: () => import("./login.page/login.page").then(m => m.LoginPage)
        },
    ]    
}