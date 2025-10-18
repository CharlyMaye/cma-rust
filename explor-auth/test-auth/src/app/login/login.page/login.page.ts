import { ChangeDetectionStrategy, Component, inject, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { Auth } from '../../security/auth';
import {MatCardModule} from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-login.page',
  imports: [MatCardModule, MatButtonModule],
  templateUrl: './login.page.html',
  styleUrl: './login.page.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
  host : { class: 'flex-full-size' }
})
export class LoginPage implements OnInit {
  private route = inject(ActivatedRoute);
  private router = inject(Router);
  private auth = inject(Auth);
  
  returnUrl: string = '/';

  ngOnInit() {
    
    // Si déjà authentifié, rediriger immédiatement
    if (this.auth.isAuthenticated()) {
      this.router.navigate([this.returnUrl]);
      return;
    }
    // Récupère l'URL de retour depuis les paramètres de query
    this.returnUrl = this.route.snapshot.queryParams?.['returnUrl'] || '/';
    console.log('Login page - Return URL:', this.returnUrl);
  }

  login() {
    console.log('Attempting login...');
    this.auth.login(this.returnUrl);

  }
}
