import { Component, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { HousingService } from '../housing.service';
import { HousingLocation } from '../housinglocation';

@Component({
  selector: 'app-new-house',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule],
  templateUrl: './new-house.component.html',
  styleUrl: './new-house.component.css'
})
export class NewHouseComponent {
  route: ActivatedRoute = inject(ActivatedRoute);
  housingService = inject(HousingService);
}
