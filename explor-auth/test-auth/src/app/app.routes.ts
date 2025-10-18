import { Routes } from '@angular/router';
import { authGuard } from './security';

export const routes: Routes = [
    // Route de login (publique) - doit être en premier
    {
        path: "login",
        loadChildren: () => import('./login').then(m => m.getRoutes())
    },
    // Routes protégées avec layout
    {
        path: "",
        loadComponent: () => import('./shared/main-layout').then(m => m.MainLayout),
        canMatch: [authGuard],
        children: [
            {
                path: "home",
                loadChildren: () => import('./home').then(m => m.getRoutes())
            },
            {
                path: "",
                redirectTo: "home",
                pathMatch: "full"
            }
        ]
    },
    // Redirection par défaut - doit être en dernier
    {
        path: "",
        redirectTo: "login",
        pathMatch: "full"
    }
];
