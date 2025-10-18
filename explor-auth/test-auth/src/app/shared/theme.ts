import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class Theme {
  private readonly STORAGE_KEY = 'color-scheme';
  private currentScheme: 'light' | 'dark' = 'light';

  constructor() {
    this.initializeTheme();
  }

  private initializeTheme(): void {
    // 1. Récupérer le scheme depuis localStorage
    const savedScheme = localStorage.getItem(this.STORAGE_KEY) as 'light' | 'dark' | null;
    
    // 2. Si pas de valeur sauvegardée, utiliser la valeur CSS calculée
    if (!savedScheme) {
      const body = document.body;
      const computedStyle = getComputedStyle(body);
      const cssScheme = computedStyle.colorScheme || 'light';
      this.currentScheme = cssScheme.includes('dark') ? 'dark' : 'light';
    } else {
      this.currentScheme = savedScheme;
    }

    // 3. Appliquer le scheme
    this.applyColorScheme(this.currentScheme);
    console.log('Theme initialized with scheme:', this.currentScheme);
  }

  private applyColorScheme(scheme: 'light' | 'dark'): void {
    document.body.style.colorScheme = scheme;
    localStorage.setItem(this.STORAGE_KEY, scheme);
  }

  public toggleColorScheme(): void {
    const newScheme = this.currentScheme === 'dark' ? 'light' : 'dark';
    this.currentScheme = newScheme;
    this.applyColorScheme(newScheme);
    console.log('Color scheme changed to:', newScheme);
  }

  public getCurrentScheme(): 'light' | 'dark' {
    return this.currentScheme;
  }
}
