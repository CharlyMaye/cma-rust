import { ApplicationConfig, inject, provideBrowserGlobalErrorListeners, provideEnvironmentInitializer, provideZonelessChangeDetection } from '@angular/core';
import { PreloadAllModules, provideRouter, withComponentInputBinding, withHashLocation, withPreloading } from '@angular/router';

import { routes } from './app.routes';
import { provideSecurity } from './security';
import { provideHttpClient, withInterceptors } from '@angular/common/http';
import { Theme } from './shared/theme';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideZonelessChangeDetection(),
    provideHttpClient(withInterceptors([])),
    provideEnvironmentInitializer(() => {
      // On force l'injection du service Theme au démarrage de l'application
      // pour appliquer le thème correct dès le début.
      inject(Theme);
    }),
    provideSecurity(),
    provideRouter(routes, withComponentInputBinding(), withHashLocation(), withPreloading(PreloadAllModules))
  ]
};
