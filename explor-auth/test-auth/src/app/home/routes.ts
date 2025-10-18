import { Routes } from "@angular/router";

export function getRoutes(): Routes {
    return [
        {
            path: "",
            loadComponent: () => import("../home/main.page/main.page").then(m => m.MainPage)
        },
    ]    
}