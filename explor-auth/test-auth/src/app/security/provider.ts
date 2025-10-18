import { EnvironmentProviders, makeEnvironmentProviders } from "@angular/core";
import { Auth, ConcreteAuth } from "./auth";
import { provideHttpClient, withInterceptors, withRequestsMadeViaParent } from "@angular/common/http";
import { authInterceptor } from "./auth-interceptor";

export function provideSecurity(): EnvironmentProviders {
    return makeEnvironmentProviders([
        provideHttpClient(withInterceptors([authInterceptor])),
        {
            provide: Auth,
            useClass: ConcreteAuth,
            multi: false
        }
    ]);
}
