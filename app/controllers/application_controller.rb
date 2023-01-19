class ApplicationController < ActionController::Base
  class ApplicationController < ActionController::Base
  before_action :configure_permitted_parameters, if: :devise_controller?
  #ユーザー認証・登録する前に(devise利用する前)↓のメソッド実行される
  protected

  def configure_permitted_parameters
    devise_parameter_sanitizer.permit(:sign_up, keys: [:email])#
  end
  end
end
