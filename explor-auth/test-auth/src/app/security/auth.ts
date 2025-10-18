import { HttpClient } from '@angular/common/http';
import { inject, Injectable, Signal, signal } from '@angular/core';
import { Router } from '@angular/router';


export abstract class Auth {
  abstract readonly isAuthenticated: Signal<boolean>;
  abstract login(): void;
  abstract logout(): void;
}

@Injectable()
export class ConcreteAuth extends Auth {
  readonly #http = inject(HttpClient);
  readonly #router = inject(Router);

  readonly #isAuthenticated = signal(false);
  readonly isAuthenticated = this.#isAuthenticated.asReadonly();

  constructor() {
    super();
    console.log('ConcreteAuth initialized');
  }

  login() {
    console.log('ConcreteAuth: User logged in');
    this.#isAuthenticated.set(true);
  }

  logout() {
    console.log('ConcreteAuth: User logged out');
    this.#isAuthenticated.set(false);
    // Rediriger vers la page de login après déconnexion
    this.#router.navigate(['/login']);
  }
}
