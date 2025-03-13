package com.integrador.MaqExpress.model;

import java.time.LocalDateTime;
import java.util.UUID;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Entity
@Table(name = "usuarios")
@Data
@AllArgsConstructor
@NoArgsConstructor
public class UserModel {
    @Id
    @GeneratedValue(strategy = GenerationType.UUID) 
    private UUID idUsuario;
    @Email(message = "Email inválido")
    @NotBlank(message = "Email vazio")
    private String email;
    private String nome;
    private String senha;
    private String documento;
    private String origemConta;
    private LocalDateTime dataCadastro;
}
