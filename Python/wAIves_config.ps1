# Commande permettant de lancer PowerShell en tant qu'administrateur
PowerShell -Command "Start-Process PowerShell -Verb RunAs"

# Modifiez la variable suivante par votre chemin d'accès à l'exécuteur Python
$PYTHON_PATH = 'C:\Chemin\Vers\Votre\Python\python.exe'

if (-not (Test-Path $PROFILE)) {
    New-Item -Path $PROFILE -ItemType File -Force
}

# Lire le contenu actuel du fichier de profil
$profileContent = Get-Content $PROFILE

# Définir la ligne à ajouter
$aliasLine = "Set-Alias python3 '$PYTHON_PATH'"

# Vérifier si la ligne existe déjà
if ($profileContent -notcontains $aliasLine) {
    # Ajouter la ligne au fichier de profil
    Add-Content -Path $PROFILE -Value $aliasLine
    Write-Host "La ligne a ete ajoutee au fichier de profil."
} else {
    Write-Host "La ligne existe deja dans le fichier de profil."
}

. $PROFILE
