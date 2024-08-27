import { Injectable } from '@angular/core';
import { HttpClient, HttpInterceptor, HttpHandler, HttpRequest, HttpEvent, HttpHandlerFn } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Person } from './person';

@Injectable({
  providedIn: 'root'
})
export class RestApiService {
  baseUrl = 'http://127.0.0.1:8000/';

  async getAllPeople(): Promise<Person[]> {
    const data = await fetch(`${this.baseUrl}people`);
    return (await data.json()) ?? [];
  }

  constructor(private http: HttpClient) {

  }

}

